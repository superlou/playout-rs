use gtk;
use relm_attributes::widget;
use relm::{Widget};
use gtk::{WidgetExt, ButtonExt};
use self::BusButtonMsg::*;
use style::Style;

#[derive(Msg)]
pub enum BusButtonMsg {
    Select,
    Deselect,
    Request,
}

#[derive(Clone)]
pub struct BusButtonModel {
    pub channel_id: u32,
    pub enabled: bool,
    pub active: bool,
}

impl Style for BusButton {}

#[widget]
impl Widget for BusButton {
    fn init_view(&mut self) {
        self.add_stylesheet(include_str!("../style/bus_button.css"));
        if self.model.active {
            self.add_class("active")
        }
    }

    fn model((channel_id, enabled, active): (u32, bool, bool)) -> BusButtonModel {
        BusButtonModel {
            channel_id: channel_id,
            enabled: enabled,
            active: active,
        }
    }

    fn update(&mut self, event: BusButtonMsg) {
        match event {
            Deselect => {
                self.model.active = false;
                self.remove_class("active");
            },
            Select => {
                self.model.active = true;
                self.add_class("active");
            },
            _ => {}
        }
    }

    view! {
        #[name="bus_button"]
        gtk::Button {
            clicked => Request,
            label: &(&self.model.channel_id + 1).to_string(),
            visible: true,
            sensitive: self.model.enabled,
            name: "bus-button",
        }
    }
}
