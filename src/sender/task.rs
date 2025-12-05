use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use crate::config::SENDER_CHANNEL_CAPACITY;

pub async fn spawn_sender(socket: Arc<UdpSocket>) -> mpsc::Sender<(Vec<u8>, SocketAddr)> {
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(SENDER_CHANNEL_CAPACITY);
    let mut counter: u64 = 0;

    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            let len = socket.send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent | ID={}", len, counter);
            counter += 1;
        }
    });

    tx
}
