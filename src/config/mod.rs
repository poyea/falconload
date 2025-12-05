mod loader;
mod types;

pub use loader::load_config;
pub use types::{
    EchoConfig, FloodConfig, LoadConfig, Mode, TargetAddress, MAX_UDP_PAYLOAD,
    SENDER_CHANNEL_CAPACITY,
};
