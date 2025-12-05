use std::io;
use std::sync::Arc;
use tokio::net::UdpSocket;

use crate::config::{FloodConfig, TargetAddress};
use crate::sender::spawn_sender;

pub async fn run(config: FloodConfig) -> io::Result<()> {
    let source_addr = config.source_addr()?;
    let target = config.target_addr()?;
    let socket = UdpSocket::bind(source_addr).await?;
    let tx = spawn_sender(Arc::new(socket)).await;

    let flood_buf = vec![config.fill_byte; config.packet_size];

    println!("Flood mode: Bound to {}", config.source_address);
    println!(
        "Flooding target: {} with {} byte packets",
        target, config.packet_size
    );

    // Flood
    loop {
        tx.send((flood_buf.clone(), target)).await.unwrap();
    }
}
