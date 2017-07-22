extern crate gst;

use std::fs;

pub struct Feed {
    pub name: String,
    pub control_pipe_name: String,
    pub width: u32,
    pub height: u32,
    pub framerate: String,
    pub pipeline: gst::Pipeline,
}

impl Feed {
    pub fn get_pipeline(&mut self) -> &mut gst::Pipeline {
        &mut self.pipeline
    }

    pub fn add_element(&mut self, element_type: &str, name: &str) {
        let element = gst::Element::new(element_type, name).unwrap();
        self.get_pipeline().add(element);
    }

    pub fn link(&mut self, src_name: &str, dest_name: &str) -> bool {
        let mut src = self.get_pipeline().get_by_name(src_name).unwrap();
        let mut dest = self.get_pipeline().get_by_name(dest_name).unwrap();
        src.link(&mut dest)
    }

    pub fn add_video_shmsink(&mut self, last_element_name: &str) {
        let mut last_element = self.get_pipeline().get_by_name(last_element_name).unwrap();
        let shm_size = self.width * self.height * 4 * 22;

        self.add_element("shmsink", "shmsink");
        let mut shm_sink = self.pipeline.get_by_name("shmsink").unwrap();

        // Remove old control pipes
        let _ = fs::remove_file(&self.control_pipe_name);

        let pipe: &str = &self.control_pipe_name;
        shm_sink.set("socket-path", pipe);
        shm_sink.set("shm-size", shm_size);
        shm_sink.set("wait-for-connection", 0);
        shm_sink.set("sync", true);

        let mixer_format = "video/x-raw, format=BGRA, pixel-aspect-ratio=1/1, interlace-mode=progressive";
        let caps_string = format!("{}, width={}, height={}, framerate={}",
                                  mixer_format,
                                  self.width,
                                  self.height,
                                  self.framerate);

        let caps = gst::Caps::from_string(&caps_string).unwrap();
        last_element.link_filtered(&mut shm_sink, &caps);
    }

    pub fn play(&mut self) {
        self.pipeline.play();
    }

    pub fn stop(&mut self) {
        self.pipeline.set_null_state();
    }
}
