pub use crate::config::*;
// pub use anyhow::Result;
pub use dns_common::prelude::*;
use dns_common::system;
pub use once_cell::sync::Lazy;
// pub use regex::Regex;

pub static PROJECT_NAME: &str = "channel";

pub type System = system::System<Config>;

pub trait Runnable {
    fn run(&self, system: System) -> Result<()>;
}
