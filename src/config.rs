use std;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

pub struct Config {
    pub master: GlobalConfig,
    //pub sockets: Vec<socket::Socket>,
    pub logging: LoggingConfig,
    pub services: Vec<ServiceConfig>,
}


#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
}


impl GlobalConfig {
}

#[derive(Deserialize, Debug)]
pub struct SocketConfig {
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServiceConfig {
}

#[derive(Deserialize, Debug)]
pub struct LoggingConfig {
}

pub fn load_config() -> Option<Config> {
   None
}

