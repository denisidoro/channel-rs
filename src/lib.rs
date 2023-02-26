extern crate dns_common;

mod components;
mod config;
pub mod led;
pub mod prelude;
pub mod server;

use dns_common::tracing;
use prelude::*;

pub fn boot(args: Option<Vec<&str>>) -> Result<()> {
    let config = Config::new(args)?;
    let cmd = config.cli.cmd.clone();

    tracing::init(config.yaml.tracing.as_ref());

    let mut system = System::new(config)?;
    system.set_type_ids(cmd.deps());
    components::init(&mut system)?;

    cmd.run(system)?;
    Ok(())
}
