mod handle;
mod config;
mod args;
use std::net::*;
use threadpool::ThreadPool;
use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = args::CliArgs::parse();
    let config = config::Config::try_from_args(args)?;

    let listener = TcpListener::bind((config.interface, config.port)).expect("Error: Failed to bind to port");
    let pool = ThreadPool::new(config.works);

    println!(
        "Serving HTTP on {} port {} (http://{}:{}/) ...",
        config.interface, config.port, config.interface, config.port
    );

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let c = config.clone_data();
            pool.execute(move || {
                let _res = handle::handle_connection(&mut stream, c);
            });
        }
    }

    Ok(())
}
