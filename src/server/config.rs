use dns_common::prelude::*;
use once_cell::sync::Lazy;
use std::env;

pub const IP: &str = "0.0.0.0";

pub fn port() -> Result<u16> {
    match env::var("PORT") {
        Ok(p) => p.parse().context("invalid $PORT"),
        Err(_) => Ok(7879),
    }
}

pub const TIME_MOD: usize = 15;
pub const DOT_ARGS: &[&str] = &["channel", "handle-tcp"];

pub static CLIENT_PASSWORD: Lazy<String> =
    Lazy::new(|| env::var("PASSWORD").expect("$PASSWORD not set"));

pub static DOTFILES: Lazy<String> = Lazy::new(|| env::var("DOTFILES").expect("$DOTFILES not set"));
