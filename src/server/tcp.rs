use super::config;
use super::security::{self, Payload};
use crate::prelude::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::process::Stdio;

pub fn serve() -> Result<()> {
    if config::CLIENT_PASSWORD.is_empty() {
        return Err(anyhow!("$PASSWORD empty"));
    }
    if config::DOTFILES.is_empty() {
        return Err(anyhow!("$DOTFILES empty"));
    }

    let address = format!("{}:{}", config::IP, config::port()?);
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if let Err(e) = handle_connection(stream) {
            eprintln!("failure: {}", e);
        }
    }

    Ok(())
}

fn get_data(stream: &mut TcpStream) -> Result<(bool, String)> {
    let mut is_http = false;
    let mut msg = String::new();

    let mut buf_reader = BufReader::new(stream);
    buf_reader.read_line(&mut msg)?;

    if msg.starts_with("GET") || msg.starts_with("POST") {
        is_http = true;
        let mut headers: HashMap<String, String> = HashMap::new();
        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line)?;
            if line.trim().is_empty() {
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                headers.insert(k.into(), v.trim().into());
            }
        }

        let len = headers
            .get("Content-Length")
            .and_then(|s| s.parse::<usize>().ok());

        if let Some(l) = len {
            let mut body = vec![];
            body.resize(l, 0);
            buf_reader.read_exact(&mut body)?;
            msg = String::from_utf8_lossy(&body).to_string();
        }
    } else {
        let mut rest = String::new();
        buf_reader.read_to_string(&mut rest)?;
        msg.push_str(&rest);
    }

    Ok((is_http, msg))
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let (is_http, msg) = get_data(&mut stream)?;

    if is_http {
        stream.write_all(
            "HTTP/1.1 200 OK
Server: channel
Content-Type: text/plain
Connection: Closed

"
            .as_bytes(),
        )?;
    }

    eprintln!("\n================\nMessage received: {msg}");
    if msg.as_str() == "ping" {
        stream.write_all("pong".as_bytes())?;
        return Ok(());
    }

    let salt = security::gen_salt()?;
    eprintln!("{salt}");

    let Payload { client, kind, data } = &security::decrypt_message(salt, &msg)?;
    eprintln!("c: {client}\nk: {kind}\nd: {data}");

    let mut args = config::DOT_ARGS.to_vec();
    args.push(client);
    args.push(kind);
    args.push(data);

    let dot_bin = format!("{}/bin/dot", &*config::DOTFILES);

    let mut child = Command::new(dot_bin)
        .stdout(Stdio::piped())
        .args(args)
        .spawn()?;

    if let Some(stdout) = &mut child.stdout {
        let lines = BufReader::new(stdout).lines();
        for line in lines {
            stream.write_all(format!("{}\n", line.unwrap_or_default()).as_bytes())?;
        }
    }

    let status = child.wait()?.code().unwrap_or(-1);

    stream.write_all(format!("status: {}\n", status).as_bytes())?;

    Ok(())
}
