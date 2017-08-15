#![feature(proc_macro)]
extern crate gst;
extern crate libc;
extern crate gdk;
extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

use relm::{Widget};
use relm_attributes::widget;
use gtk::{Inhibit, OrientableExt, WidgetExt};
use gtk::Orientation::{Horizontal, Vertical};

mod bus;
mod bus_button;
mod monitor_area;
mod style;
mod monitor;

use monitor_area::MonitorArea;
use bus::Bus;

use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> () {
    }

    fn update(&mut self, event: Msg, _model: &mut ()) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Box {
                    orientation: Horizontal,
                    MonitorArea((String::from("Program"), String::from("/tmp/mixer1"))),
                    MonitorArea((String::from("Feed 1"), String::from("/tmp/feed1-control-pipe"))),
                    MonitorArea((String::from("Feed 2"), String::from("/tmp/feed2-control-pipe"))),
                    MonitorArea((String::from("Feed 3"), String::from("/tmp/feed3-control-pipe"))),
                }
                Bus,
                Bus,
            }
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    gst::init();
    Win::run(()).unwrap();
}
