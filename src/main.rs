
use std::io;
use actix::prelude::*;

mod config;
mod global;
mod logging;

mod socket;
mod addrinfo;

fn main() {
    let sys = actix::System::new("agsps");
    let loaded = match config::load_config() {
        Some(cfg) => global::start(cfg),
        None => false,
    };
    let code = if loaded {
        println!("{:?}", loaded );
        0   //sys.run()
    } else {
        1
    };
    std::process::exit(code);
}
