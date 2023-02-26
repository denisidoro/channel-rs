use super::config;
use crate::prelude::*;
use std::time::SystemTime;
mod crypto;

fn reverse(txt: &str) -> String {
    txt.chars().rev().collect::<String>()
}

fn encrypt(txt: &str, password: &str) -> Result<String> {
    Ok(crypto::custom_vigenere(txt, &reverse(password), true))
}

fn decrypt(txt: &str, password: &str) -> Result<String> {
    Ok(crypto::custom_vigenere(txt, &reverse(password), false))
}

pub fn gen_salt() -> Result<u64> {
    let n = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    Ok(n)
}

fn effective_salt(salt: u64) -> u64 {
    let m = config::TIME_MOD as u64;
    let r = salt % m;
    if r >= m / 2 {
        salt - r + m
    } else {
        salt - r
    }
}

pub fn encrypt_message(salt: u64, client: &str, kind: &str, data: &str) -> Result<String> {
    let txt = format!("{}|{}|{}", client, kind, data);
    let password = format!("{}|{}", client, effective_salt(salt));
    let msg = format!(
        "{}|{}",
        encrypt(client, &config::CLIENT_PASSWORD)?.replace('|', "¶"),
        encrypt(&txt, &password)?
    );
    Ok(msg)
}

pub struct Payload {
    pub client: String,
    pub kind: String,
    pub data: String,
}

fn decrypt_message1(effective_salt: u64, header_client: &str, encrypted: &str) -> Result<String> {
    let password = format!("{}|{}", header_client, effective_salt);
    let decrypted = decrypt(encrypted, &password)?;
    if decrypted.starts_with(header_client) {
        Ok(decrypted)
    } else {
        Err(anyhow!("invalid password"))
    }
}

pub fn decrypt_message(salt: u64, txt: &str) -> Result<Payload> {
    let (encrypted_header_client, encrypted) = match txt.split_once('|') {
        Some(x) => Ok(x),
        None => Err(anyhow!("no client")),
    }?;
    let header_client = &decrypt(
        &encrypted_header_client.replace('¶', "|"),
        &config::CLIENT_PASSWORD,
    )?;
    let effective_salt = effective_salt(salt);

    let m = config::TIME_MOD as u64;
    let decrypted = decrypt_message1(effective_salt, header_client, encrypted)
        .or_else(|_| decrypt_message1(effective_salt + m, header_client, encrypted))
        .or_else(|_| decrypt_message1(effective_salt - m, header_client, encrypted))?;

    let mut parts = decrypted.splitn(3, '|');
    let client = parts.next().expect("no client").to_owned();
    let kind = parts.next().expect("no type").to_owned();
    let data = parts.next().expect("no data").to_owned();
    Ok(Payload { client, kind, data })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const PASSWORD: &str = "pass";

    #[test]
    fn success() {
        let salt = 1673214844;
        let client = "client";
        let kind = "ki¶nd";
        let data = "¶||data¶";

        env::set_var("PASSWORD", PASSWORD);

        let encrypted = encrypt_message(salt, client, kind, data).unwrap();

        let decrypted = decrypt_message(salt, &encrypted).unwrap();
        assert_eq!(decrypted.client, client);
        assert_eq!(decrypted.kind, kind);
        assert_eq!(decrypted.data, data);

        let decrypted = decrypt_message(salt + config::TIME_MOD as u64, &encrypted).unwrap();
        assert_eq!(decrypted.client, client);
        assert_eq!(decrypted.kind, kind);
        assert_eq!(decrypted.data, data);

        let decrypted = decrypt_message(salt - config::TIME_MOD as u64, &encrypted).unwrap();
        assert_eq!(decrypted.client, client);
        assert_eq!(decrypted.kind, kind);
        assert_eq!(decrypted.data, data);
    }

    #[test]
    fn failure() {
        let salt = 1673214844;
        let client = "client";
        let kind = "kind";
        let data = "data";

        env::set_var("PASSWORD", PASSWORD);

        let encrypted = encrypt_message(salt, client, kind, data).unwrap();
        assert!(decrypt_message(salt + (2 * config::TIME_MOD as u64) + 1, &encrypted).is_err());
    }
}
