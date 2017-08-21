#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate gst;
extern crate regex;
extern crate uuid;

pub mod feeds;
mod manager;
mod channel;
mod monitor;
mod snowmix_conn;
mod console;

pub use channel::Channel;
use std::sync::mpsc::{Sender};
pub use manager::Manager;
pub use monitor::Monitor;
pub use console::console_task;

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
