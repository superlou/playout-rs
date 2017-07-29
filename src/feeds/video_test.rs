extern crate gst;

use feeds::Feed;

pub struct VideoTest {
    feed: Feed
}

impl VideoTest {
    pub fn new(name: &str, width: u32, height: u32, framerate: &str) -> VideoTest {
        let pipeline = gst::Pipeline::new(name).unwrap();
        let control_pipe_name = format!("/tmp/{}-control-pipe", name);

        let mut feed = Feed{name: String::from(name),
                            control_pipe_name: control_pipe_name,
                            width: width,
                            height: height,
                            framerate: String::from(framerate),
                            pipeline: pipeline};

         feed.add_element("videotestsrc", "src");
         feed.add_element("videoconvert", "convert");
         feed.link("src", "convert");
         feed.add_video_shmsink("convert");

         VideoTest{feed: feed}
    }

    pub fn set_pattern(&mut self, pattern: u32) {
        let mut element = self.feed.get_element("src").unwrap();
        element.set("pattern", pattern);
    }

    pub fn play(&mut self) {
        self.feed.play();
    }

    pub fn stop(&mut self) {
        self.feed.stop();
    }
}
