use clap::{Parser, ValueHint, value_parser};

use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "fileserve", author, about, version)]
pub struct CliArgs {
    /// Be verbose, includes emitting access logs
    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Which path to serve
    #[clap(name = "PATH", value_parser=value_parser!(PathBuf), value_hint = ValueHint::AnyPath)]
    pub path: Option<PathBuf>,

    /// Port to use
    #[clap(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,

    /// Interface to listen on
    #[clap(short = 'i',long = "interfaces",value_parser=value_parser!(IpAddr), default_value = "0.0.0.0")]
    pub interface: IpAddr,

    /// works for threadpool
    #[clap(short = 'w', long = "work", default_value = "10")]
    pub works: usize,

    /// size for font
    #[clap(short = 's', long = "size", default_value = "16")]
    pub size: u32
}