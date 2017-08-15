use gtk;
use relm::{Relm, Component, Widget, Update, ContainerWidget};
use gtk::WidgetExt;
use gtk::Orientation::{Horizontal};
use bus_button::{BusButton, BusButtonModel, BusButtonMsg};

#[derive(Msg)]
pub enum BusMsg {
    Selected(u32)
}

pub struct BusModel {
}

pub struct Bus {
    model: BusModel,
    buttons: Vec<Component<BusButton>>,
    root: gtk::Box,
}

impl Update for Bus {
    type Model = BusModel;
    type ModelParam = ();
    type Msg = BusMsg;

    fn model(_: &Relm<Self>, _: ()) -> BusModel {
        let mut channels = vec![];
        channels.push(BusButtonModel{channel_id: 0, enabled: true, active: true});
        channels.push(BusButtonModel{channel_id: 1, enabled: true, active: false});
        channels.push(BusButtonModel{channel_id: 2, enabled: true, active: false});
        channels.push(BusButtonModel{channel_id: 3, enabled: true, active: false});

        BusModel {}
    }

    fn update(&mut self, event: BusMsg) {
        match event {
            BusMsg::Selected(x) => {
                for button in &mut self.buttons {
                    button.stream().emit(BusButtonMsg::Deselect);
                }
                self.buttons[x as usize].stream().emit(BusButtonMsg::Select);
            }
        }
    }
}

impl Widget for Bus {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let mut channels = vec![];
        channels.push(BusButtonModel{channel_id: 0, enabled: true, active: true});
        channels.push(BusButtonModel{channel_id: 1, enabled: true, active: false});
        channels.push(BusButtonModel{channel_id: 2, enabled: true, active: false});
        channels.push(BusButtonModel{channel_id: 3, enabled: true, active: false});

        let mut buttons = vec![];
        let hbox = gtk::Box::new(Horizontal, 0);

        for channel in channels {
            let widget = hbox.add_widget::<BusButton, _>(&relm, (channel.channel_id, channel.enabled, channel.active));
            connect!(widget@BusButtonMsg::Request, relm, BusMsg::Selected(channel.channel_id));
            buttons.push(widget);
        }

        hbox.show_all();

        Bus {
            buttons: buttons,
            model: model,
            root: hbox,
        }
    }
}
