use serde::Deserialize;
use std::io;
use std::net::SocketAddr;

/// Maximum UDP payload size
pub const MAX_UDP_PAYLOAD: usize = 65536;

/// Default sender channel capacity
pub const SENDER_CHANNEL_CAPACITY: usize = 1_000;

/// Trait for configs with target address
pub trait TargetAddress {
    fn target_address(&self) -> &str;
    fn source_address(&self) -> &str;

    fn target_addr(&self) -> io::Result<SocketAddr> {
        self.target_address()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    }

    fn source_addr(&self) -> io::Result<SocketAddr> {
        self.source_address()
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    #[serde(default)]
    pub flood: Option<FloodConfig>,
    #[serde(default)]
    pub load: Option<LoadConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EchoConfig {
    pub listen_address: String,
}

impl EchoConfig {
    pub fn listen_addr(&self) -> io::Result<SocketAddr> {
        self.listen_address
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default)]
    pub listen_address: Option<String>,
    pub mode: Mode,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Echo,
    Flood,
    Load,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FloodConfig {
    pub target_address: String,
    #[serde(default = "default_source_address")]
    pub source_address: String,
    #[serde(default = "default_packet_size")]
    pub packet_size: usize,
    #[serde(default = "default_fill_byte")]
    pub fill_byte: u8,
}

fn default_source_address() -> String {
    "0.0.0.0:0".to_string()
}

fn default_packet_size() -> usize {
    5120
}

fn default_fill_byte() -> u8 {
    0xFF
}

impl TargetAddress for FloodConfig {
    fn target_address(&self) -> &str {
        &self.target_address
    }
    fn source_address(&self) -> &str {
        &self.source_address
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoadConfig {
    pub target_address: String,
    #[serde(default = "default_source_address")]
    pub source_address: String,
    #[serde(default = "default_load_message")]
    pub message: String,
    #[serde(default = "default_interval_ms")]
    pub interval_ms: u64,
}

fn default_load_message() -> String {
    "ping".to_string()
}

fn default_interval_ms() -> u64 {
    1000
}

impl TargetAddress for LoadConfig {
    fn target_address(&self) -> &str {
        &self.target_address
    }
    fn source_address(&self) -> &str {
        &self.source_address
    }
}
