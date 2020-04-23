use std;
use std::io;
use std::net::TcpListener;
use std::error::Error;

use crate::addrinfo;
use crate::config::SocketConfig;

pub struct Socket {
    pub name: String,
    pub listener: TcpListener,
    pub info: addrinfo::AddrInfo,
}

impl Socket {

    fn new(name: String, listener: TcpListener,
           info: addrinfo::AddrInfo, cfg: &SocketConfig) -> Socket {
        let sock = TcpListener::bind("127.0.0.1:7777").unwrap();
        let addr = addrinfo::AddrInfo::new();
        Socket {
            name: "TEST".to_string(),
            listener: sock,
            info: addr
        }
    }
    //pub fn load_config(cfg: &[SocketConfig]) -> Result<Vec<Socket>, std::io::Error> {
    
    //}
}