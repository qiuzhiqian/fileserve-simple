mod handle;
mod config;
use clap::{App, Arg};
use std::net::*;
use threadpool::ThreadPool;

fn main() {
    let matches = App::new("fileserve-simple")
        .version("0.1.0")
        .arg(
            Arg::with_name("workers")
                .short("w")
                .long("workers")
                .value_name("AMOUNT")
                .help(&format!(
                    "Number of requests that can be handled concurrently [default: {}]",
                    config::DEFAULT_WORKERS
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .value_name("PORT")
                .help(&format!("Port to run on [default: {}]", config::DEFAULT_PORT))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("ADDRESS")
                .help(&format!(
                    "Alternative bind address [default: {}]",
                    config::DEFAULT_ADDRESS
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .value_name("DIRECTORY")
                .help("Alternative directory to serve [default: curent directory]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("SIZE")
                .help(&format!("set file list font size [default: {}]",config::DEFAULT_SIZE))
                .takes_value(true),
        )
        .get_matches();

    let config = config::Config{
        address: String::from(matches.value_of("address").unwrap_or(config::DEFAULT_ADDRESS)),
        port: matches.value_of("port").unwrap_or(config::DEFAULT_PORT).parse().expect("Args Error: Invalid port number"),
        works: matches.value_of("workers").unwrap_or(config::DEFAULT_WORKERS).parse().expect("Args Error: Invalid worker count"),
        root: String::from(matches.value_of("directory").unwrap_or(".")),
        size: matches.value_of("size").unwrap_or(config::DEFAULT_SIZE).parse().expect("Args Error: Invalid size number"),
    };

    let listener = TcpListener::bind((config.address.as_str(), config.port)).expect("Error: Failed to bind to port");
    let pool = ThreadPool::new(config.works);

    println!(
        "Serving HTTP on {} port {} (http://{}:{}/) ...",
        config.address, config.port, config.address, config.port
    );

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let c = config.clone_data();
            pool.execute(move || {
                let _res = handle::handle_connection(&mut stream, c);
            });
        }
    }
}
