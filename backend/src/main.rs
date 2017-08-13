#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate gst;
extern crate regex;
extern crate uuid;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod feeds;
mod manager;
mod channel;
mod monitor;
mod snowmix_conn;
mod console;

use std::path::PathBuf;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, SyncSender};
use manager::Manager;
use feeds::Feed;
use monitor::Monitor;
use console::console_task;
use channel::Channel;
use rocket::State;
use rocket_contrib::Json;

pub enum BackendMsg {
    Quit,
    Take,
    AutoTransition {secs: f32},
    SetPreview {id: i32},
    SetProgram {id: i32},
    GetServerStatus {sender: Sender<BackendResponse>},
    GetChannels {sender: Sender<BackendResponse>},
}

pub enum BackendResponse {
    OK(String),
    BusStatus {channels: Vec<Channel>},
}

#[get("/")]
fn index(sender: State<SyncSender<BackendMsg>>) -> String {
    let (response_sender, response_recevier) = mpsc::channel();
    sender.send(BackendMsg::GetServerStatus{sender: response_sender}).unwrap();
    let response = response_recevier.recv().unwrap();

    let text = match response {
        BackendResponse::OK(x) => x,
        _ => "Bad response".to_string(),
    };

    text
}

#[get("/channels")]
fn get_channels(sender: State<SyncSender<BackendMsg>>) -> Json<Vec<Channel>> {
    let (response_sender, response_recevier) = mpsc::channel();
    sender.send(BackendMsg::GetChannels{sender: response_sender}).unwrap();
    let response = response_recevier.recv().unwrap();

    let channels = match response {
        BackendResponse::BusStatus{channels: c} => c,
        _ => vec![],
    };

    Json(channels)
}

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

    let (sender, receiver) = mpsc::sync_channel::<BackendMsg>(256);
    let sender2 = sender.clone();

    thread::spawn(move || {
        rocket::ignite().manage(sender)
                        .mount("/", routes![index, get_channels])
                        .launch();
    });

    thread::spawn(move || {
        console_task(sender2);
    });

    let mut run = true;

    while run {
        let response = receiver.recv().unwrap();

        match response {
            BackendMsg::Quit => run = false,
            BackendMsg::Take => manager.take(),
            BackendMsg::AutoTransition{secs: x} => manager.transition(x),
            BackendMsg::GetServerStatus{sender: x} => {
                x.send(BackendResponse::OK("Server running".to_string())).unwrap()
            },
            BackendMsg::GetChannels{sender: x} => {
                x.send(BackendResponse::BusStatus{channels: manager.get_channels_copy()}).unwrap();
            }
            _ => {},
            // input => {
            //     match input.parse::<i32>() {
            //         Ok(id) => manager.set_preview(id as usize),
            //         Err(_) => {}
            //     }
            // }
        };
    }

    feed1.stop();
    feed2.stop();
    feed3.stop();
    monitor.stop();
    manager.quit();

    println!("Done");
}
