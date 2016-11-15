/// Data structure for configurations

use std::collections::HashMap;

/// Main configuration data structure
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// Services configuration
    pub services: HashMap<String, Service>,
}

impl Config {
    pub fn new() -> Self {
        Config { services: HashMap::new() }
    }
}

/// Individual service configuration
/// TODO: Make more generic and allow more information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// IP to listen on
    #[serde(default = "default_ip")]
    pub ip: String,

    /// IP to listen on
    #[serde(default = "default_port")]
    pub port: i64,
}

impl Service {
    pub fn new(ip: String, port: i64) -> Self {
        Service {
            ip: ip,
            port: port,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

fn default_ip() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> i64 {
    60000
}
