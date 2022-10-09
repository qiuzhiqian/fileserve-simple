pub const DEFAULT_WORKERS: &str = "10";
pub const DEFAULT_PORT: &str = "8080";
pub const DEFAULT_ADDRESS: &str = "0.0.0.0";
pub const DEFAULT_SIZE: &str = "16"; //16px

#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub works: usize,
    pub root: String,
    pub size: u32
}

impl Config {
    pub fn clone_data(&self) -> Config {
        return Config{
            address: String::from(&self.address),
            port: self.port,
            works: self.works,
            root: String::from(&self.root),
            size: self.size,
        };
    }
}