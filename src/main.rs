mod client;
mod sample;
mod local_file;
mod server;
mod util;

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(short = "b", long = "bind", default_value = "127.0.0.1:8080")]
    bind_address: String,
    #[structopt(short = "s", long = "server")]
    server_address: Option<String>,
    #[structopt(short = "f", long = "file")]
    file: Option<PathBuf>,
}

fn main() -> Result<(), String> {
    let config = Config::from_args();
    match config.server_address {
        None => match config.file {
            None => server::server(config.bind_address),
            Some(file) => local_file::local_file(file),
        }
        Some(server) => client::client(config.bind_address, server),
    }?;
    Ok(())
}
