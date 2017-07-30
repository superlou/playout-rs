use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use std::process::{Command, Child, Stdio};
use std::thread;

use regex::Regex;
use std::str;

pub struct SnowmixConn {
    addr: String,
    version: String,
    stream: TcpStream,
    child_process: Option<Child>
}

impl SnowmixConn {
    pub fn new(addr: &str) -> SnowmixConn {
        let (mut stream, child) = match TcpStream::connect(addr) {
            Ok(stream) => (stream, None),
            Err(_) => {
                println!("Snowmix server not found.  Attempting to spawn server.");
                let child = Command::new("snowmix").arg("snowmix/config.ini")
                                                   .stdin(Stdio::null())
                                                   .stdout(Stdio::null())
                                                   .stderr(Stdio::null())
                                                   .spawn()
                                                   .expect("Failed to spawn!");
                thread::sleep(Duration::from_millis(250));
                let stream = TcpStream::connect(addr).expect("Unable to spawn and connect");
                (stream, Some(child))
            }
        };

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
                     stream: stream,
                     child_process: child}
    }

    pub fn send(&mut self, msg: &str) {
        let msg = format!("{}\n", msg);
        println!("Sending: {:?}", msg);
        let _ = self.stream.write(msg.as_bytes());

        // Most valid commands won't have responses.
        // let mut response = vec![0; 4096];
        // let rx_len = self.stream.read(&mut response);
        //
        // match rx_len {
        //     Ok(len) => println!("Received: {:?}", &String::from_utf8(response).unwrap()[..len]),
        //     Err(_) => println!("No response!")
        // }
    }

    pub fn info(&self) -> String {
        let origin = match self.child_process {
            Some(_) => "spawned",
            None => "existing"
        };

        format!("Connected to {} Snowmix server at {}, version {}",
                origin, self.addr, self.version)
    }

    pub fn close(&mut self) {
        match self.child_process {
            Some(_) => self.send("quit"),
            None => {}
        }

        thread::sleep(Duration::from_millis(250));

        match self.child_process {
            Some(ref mut process) => {
                let _ = process.kill();
            },
            None => {}
        };
    }
}
