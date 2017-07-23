extern crate gst;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate regex;

mod feed;
mod video_test_feed;
mod v4l2_feed;
mod manager;
mod channel;
mod snowmix_conn;

use std::io;

use video_test_feed::VideoTestFeed;
use v4l2_feed::V4L2Feed;
use manager::Manager;

fn main() {
    gst::init();

    let mut feed1 = VideoTestFeed::new("feed1", 1280, 720, "30/1");
    feed1.play();

    let mut feed2 = V4L2Feed::new("feed2", 1280, 720, "30/1", "/dev/video0");
    feed2.play();

    let mut manager = Manager::new("127.0.0.1:9999");
    manager.start();

    let mut run = true;

    while run {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "q" | "quit" => run = false,
            "" => manager.take(),
            "a" => manager.transition(0.25),
            input => {
                match input.parse::<i32>() {
                    Ok(id) => manager.set_preview(id as usize),
                    Err(_) => {}
                }
            }
        };
    }

    feed1.stop();
    feed2.stop();

    println!("Done");
}
