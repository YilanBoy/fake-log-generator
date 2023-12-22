use std::fs;
use std::net::Ipv4Addr;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConnectionSettings {
    pub host: Host,
}

#[derive(Deserialize)]
pub struct Host {
    pub protocol: String,
    pub ip: Ipv4Addr,
    pub ports: Vec<u16>,
    pub total_requests: u32,
}


pub fn read_connection_settings_file() -> String {
    let content = fs::read_to_string("connection_settings.toml");

    let content = match content {
        Ok(content) => content,
        Err(_) => panic!("Failed to read connection_settings.toml"),
    };

    content
}

pub fn parse_connection_settings(content: String) -> ConnectionSettings {
    let config: ConnectionSettings = toml::from_str(&content).unwrap();

    config
}