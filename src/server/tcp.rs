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

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut msg = String::new();
    buf_reader.read_to_string(&mut msg)?;

    eprintln!("\n================\nMessage received");
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
