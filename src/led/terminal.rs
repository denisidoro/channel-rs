use super::{payload, Device};
use crate::prelude::*;
use payload::{Command, Payload};

pub fn main(commands: Vec<Command>) -> Result<()> {
    for command in commands {
        match command {
            Command::Wait(w) => println!("w{}", w),
            _ => {
                let payload: Payload = command.into();
                for b in payload {
                    print!("{:02x}", b);
                }
                println!();
            }
        }
    }
    Ok(())
}
