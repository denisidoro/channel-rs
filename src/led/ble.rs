use super::{payload, Device};
use crate::prelude::*;
use btleplug;
use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter,
};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use payload::{Command, Payload};
use std::time::Duration;
use tokio;

pub async fn main(commands: Vec<Command>, device: Device) -> Result<()> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await.unwrap();
    let central = adapters.into_iter().next().context("no central")?;
    let mut events = central.events().await?;

    info!("starting scan");
    central.start_scan(ScanFilter::default()).await?;

    let mut processed = false;

    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(id) = event {
            trace!(msg = "device discovered", id = id.to_string());
            let peripheral_id = device.peripheral.to_string();
            if id.to_string() == peripheral_id {
                info!("LED found");
                central.stop_scan().await?;
                let peripherals = central.peripherals().await?;
                let peripheral = peripherals
                    .iter()
                    .find(|p| p.id().to_string() == peripheral_id)
                    .context("no peripheral")?;
                process(peripheral, commands, device).await?;
                processed = true;
                break;
            }
        }
    }

    if processed {
        Ok(())
    } else {
        Err(anyhow!("didn't run command: main"))
    }
}

async fn process(peripheral: &Peripheral, commands: Vec<Command>, device: Device) -> Result<()> {
    trace!("process");
    peripheral.connect().await?;
    peripheral.discover_services().await?;

    for service in peripheral.services() {
        for characteristic in service.characteristics {
            if service.uuid == device.service && characteristic.uuid == device.characteristic {
                run_commands(peripheral, characteristic, commands).await?;
                return Ok(());
            } else {
                trace!(
                    msg = "skipped device",
                    service = service.uuid.to_string(),
                    characteristic = characteristic.uuid.to_string()
                );
            }
        }
    }

    Err(anyhow!("didn't run command: process"))
}

async fn run_commands(
    peripheral: &Peripheral,
    characteristic: Characteristic,
    commands: Vec<Command>,
) -> Result<()> {
    for command in commands {
        match command {
            Command::Wait(w) => tokio::time::sleep(Duration::from_secs(w as u64)).await,
            _ => {
                info!(msg = "sending data", command = format!("{:?}", &command));
                let payload: Payload = command.into();
                peripheral
                    .write(
                        &characteristic,
                        &payload,
                        btleplug::api::WriteType::WithoutResponse,
                    )
                    .await?;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    Ok(())
}
