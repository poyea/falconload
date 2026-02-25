use std::io;

pub mod config;
pub mod sdk;
pub mod sender;
pub mod server;

use config::{load_config, EchoConfig, Mode};

pub async fn run_with_config(config_path: &str) -> io::Result<()> {
    let config = load_config(config_path)?;

    match config.server.mode {
        Mode::Echo => {
            let listen_address = config
                .server
                .listen_address
                .expect("Echo mode requires listen_address");
            let echo_config = EchoConfig { listen_address };
            server::run_echo(echo_config).await
        }
        Mode::Flood => {
            let flood_config = config
                .flood
                .expect("Flood mode requires [flood] configuration");
            server::run_flood(flood_config).await
        }
        Mode::Load => {
            let load_config = config
                .load
                .expect("Load mode requires [load] configuration");
            server::run_load(load_config).await
        }
    }
}
