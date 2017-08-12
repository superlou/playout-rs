use gtk;
use relm::{Component, Widget, RemoteRelm, ContainerWidget};
use gtk::WidgetExt;
use gtk::Orientation::{Horizontal};
use bus_button::{BusButton, BusButtonModel, BusButtonMsg};

#[derive(Msg)]
pub enum BusMsg {
    Selected(u32)
}

#[derive(Clone)]
pub struct BusModel {
    channels: Vec<BusButtonModel>
}

#[derive(Clone)]
pub struct Bus {
    buttons: Vec<Component<BusButton>>,
    root: gtk::Box,
}

impl Widget for Bus {
    type Model = BusModel;
    type ModelParam = ();
    type Msg = BusMsg;
    type Root = gtk::Box;

    fn model(_: ()) -> BusModel {
        let mut channels = vec![];
        channels.push(BusButtonModel{channel_id: 0, enabled: true, active: true});
        channels.push(BusButtonModel{channel_id: 1, enabled: true, active: false});
        channels.push(BusButtonModel{channel_id: 2, enabled: true, active: false});
        channels.push(BusButtonModel{channel_id: 3, enabled: true, active: false});

        BusModel {
            channels: channels
        }
    }

    fn root(&self) -> &Self::Root {
        &self.root
    }

    fn update(&mut self, event: BusMsg, _model: &mut BusModel) {
        match event {
            BusMsg::Selected(x) => {
                for button in &mut self.buttons {
                    button.stream().emit(BusButtonMsg::Deselect);
                }
                self.buttons[x as usize].stream().emit(BusButtonMsg::Select);
            }
        }
    }

    fn view(relm: &RemoteRelm<Self>, model: &BusModel) -> Self {
        let mut buttons = vec![];
        let hbox = gtk::Box::new(Horizontal, 0);

        let model = model.clone();

        for channel in model.channels {
            let widget = hbox.add_widget::<BusButton, _>(&relm, (channel.channel_id, channel.enabled, channel.active));
            connect!(widget@BusButtonMsg::Request, relm, BusMsg::Selected(channel.channel_id));
            buttons.push(widget);
        }

        hbox.show_all();

        Bus {
            buttons: buttons,
            root: hbox,
        }
    }
}
