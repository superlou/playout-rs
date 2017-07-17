use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

use snowmix_conn::SnowmixConn;

pub struct Manager {
    snowmix: SnowmixConn,
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
