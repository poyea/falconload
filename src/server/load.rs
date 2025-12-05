use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::interval;

use crate::config::{LoadConfig, TargetAddress};
use crate::sender::spawn_sender;

pub async fn run(config: LoadConfig) -> io::Result<()> {
    let source_addr = config.source_addr()?;
    let target = config.target_addr()?;
    let socket = UdpSocket::bind(source_addr).await?;
    let tx = spawn_sender(Arc::new(socket)).await;

    let message = config.message.as_bytes().to_vec();

    println!("Load mode: Bound to {}", config.source_address);
    println!(
        "Sending '{}' to {} every {}ms",
        config.message, target, config.interval_ms
    );

    let mut ticker = interval(Duration::from_millis(config.interval_ms));

    loop {
        ticker.tick().await;
        tx.send((message.clone(), target)).await.unwrap();
    }
}
