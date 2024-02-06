use std::net::SocketAddr;

use super::Config;

#[derive(serde_wrapper::Deserialize)]
pub struct NetworkConfig {
    pub api: ApiConfig,
}

#[derive(serde_wrapper::Deserialize)]
pub struct ApiConfig {
    http_url: String,
    http_port: u16,
}

impl ApiConfig {
    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::new(
            self.http_url.parse().expect("url is invalid"),
            self.http_port,
        )
    }
}

impl Config for NetworkConfig {
    const PREFIX: &'static str = "NETWORK";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config() {
        std::env::set_var("NEXUS_NETWORK_API_HTTPURL", "127.0.0.1");
        std::env::set_var("NEXUS_NETWORK_API_HTTPPORT", "9999");

        let config = <NetworkConfig as Config>::from_env().unwrap();
        config.api.bind_addr();
    }
}