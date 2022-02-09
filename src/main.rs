use crate::shutdown::Shutdown;
use std::process;
use std::time::Duration;
use time::sleep;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::{broadcast, mpsc};
use tokio::sync::mpsc::Sender;
use tokio::time;

pub mod shutdown;

pub async fn wait_seconds(seconds: u64) -> anyhow::Result<()> {
    sleep(Duration::from_secs(seconds)).await;
    Ok(())
}

pub async fn operation(mut shutdown: Shutdown, _sender: Sender<&str>) -> anyhow::Result<()> {
    println!("operation started ...");
    sleep(Duration::from_secs(2)).await;
    if !shutdown.is_shutdown() {
        let _ = tokio::select! {
            res = wait_seconds(5) => res?,
            _ = shutdown.recv() => {
                wait_seconds(5).await;
                return Ok(())
            }
        };
    }
    println!("operation finished ...");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel::<&str>(1);

    println!("My pid is {}", process::id());
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::select! {
        _ = async {
            loop {
                println!("loop start...");
                let shutdown = Shutdown::new(notify_shutdown.subscribe());
                let tx = shutdown_complete_tx.clone();
                sleep(Duration::from_secs(1)).await;
                tokio::spawn(async {
                    operation(shutdown, tx).await
                });
                println!("loop end...");
            }
        } => {
            println!("done");
        }
        _ = sigterm.recv() => {
            println!("receive SIGTERM");
        }
    }
    drop(notify_shutdown);
    drop(shutdown_complete_tx);
    println!("before shutdown_complete_rx receive.");
    let _ = shutdown_complete_rx.recv().await;
    println!("terminating the process...");
    Ok(())
}
