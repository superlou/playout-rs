use std::fmt;
use snowmix_conn::SnowmixConn;
use channel::Channel;

pub struct Manager {
    snowmix: SnowmixConn,
    channels: Vec<Channel>
}

impl Manager {
    pub fn new(snowmix_addr: &str) -> Manager {
        let snowmix = SnowmixConn::new(snowmix_addr);
        let mut channels: Vec<Channel> = Vec::new();

        for i in 0..8 {
            channels.push(Channel {
                id: i,
                snowmix_id: i + 1,
                label: format!("{}", i + 1),
                is_preview: false,
                is_program: false,
                is_dsk: false
            });
        }

         Manager{snowmix: snowmix,
                 channels: channels}
    }

    pub fn run(&mut self) {
        println!("{}", self.snowmix.info());
        self.set_program(0);
        self.set_preview(1);
    }

    fn set_program(&mut self, channel_id: usize) {
        match self.get_program() {
            Some(x) => {x.is_program = false},
            None => {}
        }

        self.channels[channel_id].is_program = true;
        self.update_main_bus();
    }

    fn set_preview(&mut self, channel_id: usize) {
        match self.get_preview() {
            Some(x) => {x.is_preview = false},
            None => {}
        }

        self.channels[channel_id].is_preview = true;
        self.update_main_bus();
    }

    fn update_main_bus(&mut self) {
        let program_id = match self.get_program() {
            Some(x) => x.snowmix_id,
            None => return,
        };

        let preview_id = match self.get_preview() {
            Some(x) => x.snowmix_id,
            None => return,
        };

        let dsk_feeds_list = self.build_dsks_feeds_list();

        self.snowmix.send(&format!("vfeed alpha {} 0", program_id));
        self.snowmix.send(&format!("vfeed alpha {} 1", preview_id));
        self.snowmix.send(&format!("tcl eval SetFeedToOverlay {} {} {}",
                                   program_id,
                                   preview_id,
                                   dsk_feeds_list));
    }

    fn build_dsks_feeds_list(&self) -> String {
        let dsks: Vec<String> = self.channels.iter().filter_map(|ref channel| {
            match channel.is_dsk {
                true => Some(channel.snowmix_id.to_string()),
                false => None
            }
        }).collect();

        dsks.join(" ")
    }

    fn get_program(&mut self) -> Option<&mut Channel> {
        for channel in self.channels.iter_mut() {
            if channel.is_program {
                return Some(&mut *channel);
            }
        }
        None
    }

    fn get_preview(&mut self) -> Option<&mut Channel> {
        for channel in self.channels.iter_mut() {
            if channel.is_preview {
                return Some(&mut *channel);
            }
        }
        None
    }
}