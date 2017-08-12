extern crate gst;
extern crate regex;
extern crate uuid;

mod feeds;
mod manager;
mod channel;
mod monitor;
mod snowmix_conn;

use std::io;
use std::path::PathBuf;
use manager::Manager;
use feeds::Feed;
use monitor::Monitor;

fn main() {
    gst::init();

    let config = feeds::VideoConfig{width: 1280,
                                    height: 720,
                                    framerate: "30/1".to_string()};

    let mut feed1 = feeds::VideoTest::new("feed1", &config);
    feed1.set_pattern(feeds::Pattern::Ball);
    let mut feed2 = feeds::V4L2::new("feed2", &config, "/dev/video0");
    let mut feed3 = feeds::CG::new("feed3", &config);
    feed3.create_slide(PathBuf::from("media/lower_third.svg"));
    feed3.select_slide(0);

    feed1.play();
    feed2.play();
    feed3.play();

    let mut manager = Manager::new("127.0.0.1:9999");
    manager.start();

    let mut monitor = Monitor::new("/tmp/mixer1", config);
    monitor.play();

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
    feed3.stop();
    monitor.stop();
    manager.quit();

    println!("Done");
}
