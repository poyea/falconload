use std::io;
use std::sync::Arc;
use tokio::net::UdpSocket;

use crate::config::{EchoConfig, MAX_UDP_PAYLOAD};
use crate::sender::spawn_sender;

pub async fn run(config: EchoConfig) -> io::Result<()> {
    let listen_addr = config.listen_addr()?;
    let socket = UdpSocket::bind(listen_addr).await?;
    let r = Arc::new(socket);
    let tx = spawn_sender(r.clone()).await;

    println!("Echo mode: Listening at {}", config.listen_address);

    loop {
        let mut buf = [0; MAX_UDP_PAYLOAD];
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        // Echo
        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}
