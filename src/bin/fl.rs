use clap::Parser;
use std::io;

use falconload::run_with_config;

#[derive(Parser, Debug)]
#[command(name = "fl")]
#[command(about = "Alias for falconload", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    run_with_config(&args.config).await
}
