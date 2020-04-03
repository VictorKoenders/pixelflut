use std::net::{IpAddr, Ipv4Addr};

#[derive(Clone)]
pub struct Config {
    pub max_threads: usize,
    pub host: IpAddr,
    pub port: u16,
    pub screen_dimensions: (u16, u16),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_threads: 4,
            host: Ipv4Addr::LOCALHOST.into(),
            port: 1234,
            screen_dimensions: (800, 600),
        }
    }
}
