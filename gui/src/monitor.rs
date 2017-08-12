use gdk;
use gtk;
use gtk::{WidgetExt};
use relm::{Component, Widget, RemoteRelm};

#[derive(Msg)]
pub enum MonitorMsg {
    Realized
}

#[derive(Clone)]
pub struct MonitorModel {
}

#[derive(Clone)]
pub struct Monitor {
    root: gtk::DrawingArea
}

extern {
    fn gdk_x11_window_get_xid(window: gdk::Window) -> u32;
}


impl Widget for Monitor {
    type Model = MonitorModel;
    type ModelParam = ();
    type Msg = MonitorMsg;
    type Root = gtk::DrawingArea;

    fn model(_: ()) -> MonitorModel {
        MonitorModel {}
    }

    fn root(&self) -> &Self::Root {
        &self.root
    }

    fn update(&mut self, event: MonitorMsg, _model: &mut MonitorModel) {
        match event {
            MonitorMsg::Realized => {
                let window = self.root().get_window().unwrap();
                println!("{:?}", window);
                unsafe {
                    let xid = gdk_x11_window_get_xid(window);
                    println!("xid: {}", xid);
                }
            }
        }
    }

    fn view(relm: &RemoteRelm<Self>, model: &MonitorModel) -> Self {
        let mut area = gtk::DrawingArea::new();
        area.set_size_request(640, 480);
        area.set_visible(true);
        connect!(relm, area, connect_realize(_), MonitorMsg::Realized);

        Monitor {
            root: area
        }
    }
}
