extern crate gst;

use feeds::Feed;

pub enum Pattern {
    SMPTE, Snow, Black, White, Red, Green, Blue, Checkers1, Checkers2, Checkers4,
    Checkers8, Circular, Blink, SMPTE75, ZonePlate, Gamut, ChromaZonePlate, Solid,
    Ball, SMPTE100, Bar, Pinwheel, Spokes, Gradient, Colors
}

pub struct VideoTest {
    name: String,
    control_pipe_name: String,
    width: u32,
    height: u32,
    framerate: String,
    pipeline: gst::Pipeline,
}

impl VideoTest {
    pub fn new(name: &str, width: u32, height: u32, framerate: &str) -> VideoTest {
        let pipeline = gst::Pipeline::new(name).unwrap();
        let control_pipe_name = format!("/tmp/{}-control-pipe", name);

        let mut feed = VideoTest{name: String::from(name),
                                 control_pipe_name: control_pipe_name,
                                 width: width,
                                 height: height,
                                 framerate: String::from(framerate),
                                 pipeline: pipeline};

         feed.add_element("videotestsrc", "src");
         feed.add_element("videoconvert", "convert");
         feed.link("src", "convert");
         feed.add_video_shmsink("convert");

         feed
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        let mut element = self.get_element("src").unwrap();
        element.set("pattern", pattern as u32);
    }
}

impl Feed for VideoTest {
    fn get_pipeline(&mut self) -> &mut gst::Pipeline {
        &mut self.pipeline
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn get_framerate(&self) -> &String {
        &self.framerate
    }

    fn get_control_pipe_name(&self) -> &String {
        &self.control_pipe_name
    }
}
