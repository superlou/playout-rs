use gtk;
use relm_attributes::widget;
use relm::{Widget};
use gtk::{WidgetExt, ButtonExt, OrientableExt};
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
    fn init_view(&self, model: &mut BusButtonModel) {
        self.add_stylesheet(include_str!("../style/bus_button.css"));
        if model.active {
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

    fn update(&mut self, event: BusButtonMsg, model: &mut BusButtonModel) {
        match event {
            Deselect => {
                model.active = false;
                self.remove_class("active");
            },
            Select => {
                model.active = true;
                self.add_class("active");
            },
            _ => {}
        }
    }

    view! {
        #[name="bus_button"]
        gtk::Button {
            clicked => Request,
            label: &(&model.channel_id + 1).to_string(),
            visible: true,
            sensitive: model.enabled,
            name: "bus-button",
        }
    }
}
