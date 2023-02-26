use crate::prelude::*;
use clap::{Parser, Subcommand};
use dns_common_derive::{HasDeps, Runnable};

#[derive(Subcommand, Debug, Clone, Runnable, HasDeps)]
pub enum Cmd {
    Config(super::command::Input),
    Server(crate::server::command::Input),
    Led(crate::led::command::Input),
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct ClapConfig {
    #[clap(subcommand)]
    pub cmd: Cmd,

    #[clap(short, long)]
    pub config: Option<PathBuf>,
}

impl ClapConfig {
    pub fn new(args: Option<Vec<&str>>) -> Result<Self> {
        // dbg!(&args);
        match args {
            Some(a) => Self::try_parse_from(&a).map_err(|e| e.into()),
            None => Self::try_parse().map_err(|e| e.into()),
        }
    }
}
