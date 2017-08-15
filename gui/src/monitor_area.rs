use gdk;
use gtk;
use gtk::{WidgetExt, OrientableExt};
use relm_attributes::widget;
use relm::{Widget};
use monitor::{VideoConfig, Monitor, Feed};
use std::rc::Rc;
use gtk::Orientation::Vertical;

#[derive(Msg)]
pub enum MonitorAreaMsg {
    Realized
}

pub struct MonitorAreaModel {
    label: String,
    feed_path: String,
    monitor: Option<Rc<Monitor>>
}

extern {
    fn gdk_x11_window_get_xid(window: gdk::Window) -> u32;
}

#[widget]
impl Widget for MonitorArea {
    fn init_view(&mut self) {
        self.drawing_area.set_size_request(356, 200);
    }

    fn model(params: (String, String)) -> MonitorAreaModel {
        MonitorAreaModel {label: params.0, feed_path: params.1, monitor: None}
    }

    fn update(&mut self, event: MonitorAreaMsg) {
        match event {
            MonitorAreaMsg::Realized => {
                let feed_path = self.model.feed_path.clone();
                let monitor = self.create_monitor(&feed_path);
                self.model.monitor = monitor;
            }
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            #[name="drawing_area"]
            gtk::DrawingArea {
                realize => MonitorAreaMsg::Realized
            },
            gtk::Label {
                text: &self.model.label
            }
        }
    }
}

impl MonitorArea {
    fn get_xid(&mut self) -> u32 {
        let window = self.drawing_area.get_window().unwrap();
        unsafe {
            gdk_x11_window_get_xid(window)
        }
    }

    fn create_monitor(&mut self, socket_path: &str) -> Option<Rc<Monitor>> {
        // todo This config should not be hard-coded.
        // Config should come from backend.
        let config = VideoConfig{width: 1280,
                                 height: 720,
                                 framerate: "30/1".to_string()};
        let mut monitor = Monitor::new(socket_path, &config);
        monitor.set_window_xid(self.get_xid());
        monitor.play();
        Some(Rc::new(monitor))
    }
}
