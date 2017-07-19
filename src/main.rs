extern crate gst;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate regex;

mod feed;
mod video_test_feed;
mod manager;
mod channel;
mod snowmix_conn;

use video_test_feed::VideoTestFeed;
use manager::Manager;

fn main() {
    // gst::init();
    // let mut feed = VideoTestFeed::new("feed1", 1280, 720, "30/1");
    // feed.play();
    //
    // loop {}
    //
    // feed.stop();

    let mut manager = Manager::new("127.0.0.1:9999");
    manager.run();

    println!("Done");
}
