use clap::Parser;
use std::io;

mod config;
mod sender;
mod server;

use config::{load_config, EchoConfig, Mode};

#[derive(Parser, Debug)]
#[command(name = "hayabusa")]
#[command(about = "Lightweight UDP-based load testing toolkit", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let config = load_config(&args.config)?;

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
