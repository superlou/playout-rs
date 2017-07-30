extern crate gst;

pub mod v4l2;
pub mod video_test;
pub mod cg;

use std::fs;
use gst::Element;
pub use self::v4l2::V4L2;
pub use self::video_test::{VideoTest, Pattern};
pub use self::cg::CG;

pub struct VideoConfig {
    pub width: u32,
    pub height: u32,
    pub framerate: String,
}

pub trait Feed {
    fn get_pipeline(&mut self) -> &mut gst::Pipeline;
    fn get_dimensions(&self) -> (u32, u32);
    fn get_framerate(&self) -> &String;
    fn get_control_pipe_name(&self) -> &String;

    fn add_element(&mut self, element_type: &str, name: &str) {
        let element = gst::Element::new(element_type, name).unwrap();
        self.get_pipeline().add(element);
    }

    fn get_element(&mut self, name: &str) -> Option<Element> {
        self.get_pipeline().get_by_name(name)
    }

    fn link(&mut self, src_name: &str, dest_name: &str) -> bool {
        let mut src = self.get_pipeline().get_by_name(src_name).unwrap();
        let mut dest = self.get_pipeline().get_by_name(dest_name).unwrap();
        src.link(&mut dest)
    }

    fn add_video_shmsink(&mut self, last_element_name: &str) {
        let mut last_element = self.get_pipeline().get_by_name(last_element_name).unwrap();
        let (width, height) = self.get_dimensions();
        let shm_size = width * height * 4 * 22;

        self.add_element("shmsink", "shmsink");
        let mut shm_sink = self.get_pipeline().get_by_name("shmsink").unwrap();

        // Remove old control pipes
        let _ = fs::remove_file(&self.get_control_pipe_name());

        shm_sink.set("socket-path", &self.get_control_pipe_name() as &str);
        shm_sink.set("shm-size", shm_size);
        shm_sink.set("wait-for-connection", 0);
        shm_sink.set("sync", true);

        let mixer_format = "video/x-raw, format=BGRA, pixel-aspect-ratio=1/1, interlace-mode=progressive";
        let caps_string = format!("{}, width={}, height={}, framerate={}",
                                  mixer_format,
                                  width,
                                  height,
                                  self.get_framerate());

        let caps = gst::Caps::from_string(&caps_string).unwrap();
        last_element.link_filtered(&mut shm_sink, &caps);
    }

    fn play(&mut self) {
        self.get_pipeline().play();
    }

    fn stop(&mut self) {
        self.get_pipeline().set_null_state();
    }
}
