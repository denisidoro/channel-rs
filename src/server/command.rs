use super::security;
use super::tcp;
use crate::prelude::*;
use clap::Args;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum SubCmd {
    Serve,
    Encrypt {
        client: String,
        kind: String,
        data: String,
    },
    Decrypt {
        msg: String,
    },
    Salt,
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

impl Runnable for Input {
    fn run(&self, _system: System) -> Result<()> {
        use SubCmd::*;
        match &self.subcmd {
            Serve => tcp::serve(),
            Encrypt { client, kind, data } => {
                let salt = security::gen_salt()?;
                let encrypted = security::encrypt_message(salt, client, kind, data)?;
                println!("{encrypted}");
                Ok(())
            }
            Decrypt { msg } => {
                let salt = security::gen_salt()?;
                let decrypted = security::decrypt_message(salt, msg)?;
                println!(
                    "client: {}, type: {}, data: {}",
                    decrypted.client, decrypted.kind, decrypted.data
                );
                Ok(())
            }
            Salt => {
                println!("{}", security::gen_salt()?);
                Ok(())
            }
        }
    }
}

impl HasDeps for Input {}
