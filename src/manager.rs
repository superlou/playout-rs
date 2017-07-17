use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

use regex::Regex;

pub struct Manager {
    snowmix: SnowmixConn,
}

pub struct SnowmixConn {
    addr: String,
    version: String,
    stream: TcpStream
}

impl Manager {
    pub fn new(snowmix_addr: &str) -> Manager {
        let snowmix = SnowmixConn::new(snowmix_addr);
         Manager{snowmix: snowmix}
    }

    pub fn run(&mut self) {
        println!("{}", self.snowmix.info());
        // self.set_program(0);
        // self.set_preview(1);
        self.snowmix.send("stack 0 1");
    }
}

impl SnowmixConn {
    fn new(addr: &str) -> SnowmixConn {
        let mut stream = TcpStream::connect(addr)
                                    .expect("Unable to connect to Snowmix server");

        // Version string is received immediately on connecting
        let mut response = vec![0; 4096];
        let response_len = stream.read(&mut response).unwrap();

        if response_len == 0 {
            panic!("Expected Snowmix server to respond with version.");
        }

        let response = String::from_utf8(response).unwrap();
        let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
        let mat = re.find(&response).unwrap();
        let version = &response[mat.start()..mat.end()];

        let _ = stream.set_read_timeout(Some(Duration::new(1, 0)));

        SnowmixConn {addr: String::from(addr),
                     version: String::from(version),
                     stream: stream}
    }

    fn send(&mut self, msg: &str) {
        let msg = format!("{}\n", msg);
        let _ = self.stream.write(msg.as_bytes());

        let mut response = vec![0; 4096];
        let _ = self.stream.read(&mut response).unwrap();
        // println!("{}", String::from_utf8(response).unwrap());
    }

    fn info(&mut self) -> String {
        format!("Connected to Snowmix server at {}, version {}", self.addr, self.version)
    }
}
