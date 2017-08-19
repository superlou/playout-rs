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
mod monitor_grid;
mod style;
mod monitor;

use monitor_area::{MonitorArea, MonitorAreaMsg};
use monitor_grid::{MonitorGrid, MonitorGridMsg};
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

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    fn init_view(&mut self) {
        self.program_monitor.emit(MonitorAreaMsg::SetLabelAndPath(
            "Program".to_string(),
            "/tmp/mixer1".to_string(),
        ));

        self.monitors.emit(MonitorGridMsg::SetMonitor(
            0, "Feed 1".to_string(), "/tmp/feed1-control-pipe".to_string()
        ));

        self.monitors.emit(MonitorGridMsg::SetMonitor(
            1, "Feed 2".to_string(), "/tmp/feed2-control-pipe".to_string()
        ));

        self.monitors.emit(MonitorGridMsg::SetMonitor(
            2, "Feed 3".to_string(), "/tmp/feed3-control-pipe".to_string()
        ));

        self.monitors.emit(MonitorGridMsg::SetMonitor(
            3, "Feed 4".to_string(), "/tmp/feed4-control-pipe".to_string()
        ));

        self.monitors.emit(MonitorGridMsg::SetMonitor(
            4, "Feed 5".to_string(), "/tmp/feed5-control-pipe".to_string()
        ));

        self.monitors.emit(MonitorGridMsg::SetMonitor(
            5, "Feed 6".to_string(), "/tmp/feed6-control-pipe".to_string()
        ));
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Box {
                    orientation: Horizontal,
                    #[name="monitors"]
                    MonitorGrid(3, 2),
                    #[name="program_monitor"]
                    MonitorArea,
                },
                Bus,
                Bus,
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    gst::init();
    Win::run(()).unwrap();
}
