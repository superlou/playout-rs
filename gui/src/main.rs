#![feature(proc_macro)]

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
use gtk::Orientation::{Vertical};

mod bus;
mod bus_button;
mod monitor;
mod style;

use monitor::Monitor;
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
                Monitor,
                Bus,
                Bus,
            }
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
