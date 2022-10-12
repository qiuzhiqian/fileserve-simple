use crate::args::CliArgs;

use std::net::{IpAddr};
use std::path::PathBuf;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    /// Enable verbose mode
    pub verbose: bool,
    /// Path to be served by miniserve
    pub path: std::path::PathBuf,
    /// Port on which miniserve will be listening
    pub port: u16,
    /// IP address(es) on which miniserve will be available
    pub interface: IpAddr,
    pub works: usize,
    pub size: u32
}

impl Config {
    pub fn clone_data(&self) -> Config {
        Config{
            verbose: self.verbose,
            port: self.port,
            works: self.works,
            path: Path::new(self.path.to_str().unwrap_or(".")).to_path_buf(),
            interface: self.interface,
            size: self.size,
        }
    }

    /// Parses the command line arguments
    pub fn try_from_args(args: CliArgs) -> std::io::Result<Self> {
        Ok(Config {
            verbose: args.verbose,
            path: args.path.unwrap_or_else(|| PathBuf::from(".")),
            port: args.port,
            interface: args.interface,
            works: args.works,
            size: args.size,
        })
    }
}
