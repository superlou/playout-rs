use gtk;
use gtk::{WidgetExt, OrientableExt, ContainerExt};
use relm_attributes::widget;
use relm::{Widget, Component, ContainerWidget, Relm};
use gtk::Orientation::{Horizontal, Vertical};
use monitor_area::{MonitorArea, MonitorAreaMsg};

#[derive(Msg)]
pub enum MonitorGridMsg {
    SetMonitor(usize, String, String),
}

pub struct MonitorGridModel {
    rows: i32,
    cols: i32,
    monitor_areas: Vec<Component<MonitorArea>>,
    relm: Relm<MonitorGrid>,
}

#[widget]
impl Widget for MonitorGrid {
    fn model(relm: &Relm<Self>, params: (i32, i32)) -> MonitorGridModel {
        let monitor_areas = vec![];
        MonitorGridModel{cols: params.0,
                         rows: params.1,
                         monitor_areas: monitor_areas,
                         relm: relm.clone()}
    }

    fn init_view(&mut self) {
        for i in 0..self.model.rows {
            let row = gtk::Box::new(Horizontal, 0);
            self.grid.add(&row);

            for j in 0..self.model.cols {
                let widget = row.add_widget::<MonitorArea, _>(&self.model.relm, ());
                self.model.monitor_areas.push(widget);
            }

            self.grid.show_all();
        }
    }

    fn update(&mut self, event: MonitorGridMsg) {
        match event {
            MonitorGridMsg::SetMonitor(i, label, path) => {
                self.set_label_and_path(i, &label, &path);
            }
        }
    }

    view! {
        #[name="grid"]
        gtk::Box {
            orientation: Vertical,
        }
    }

    fn set_label_and_path(&mut self, index: usize, label: &str, path: &str) {
        self.model.monitor_areas[index].emit(MonitorAreaMsg::SetLabelAndPath(
            label.to_string(),
            path.to_string(),
        ));
    }
}
