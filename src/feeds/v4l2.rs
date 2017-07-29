extern crate gst;

use feeds::Feed;

pub struct V4L2 {
    feed: Feed,
    device: String
}

impl V4L2 {
    pub fn new(name: &str, width: u32, height: u32, framerate: &str, device: &str) -> V4L2 {
        let pipeline = gst::Pipeline::new(name).unwrap();
        let control_pipe_name = format!("/tmp/{}-control-pipe", name);

        let mut feed = Feed{name: String::from(name),
                            control_pipe_name: control_pipe_name,
                            width: width,
                            height: height,
                            framerate: String::from(framerate),
                            pipeline: pipeline};

        feed.add_element("v4l2src", "src");
        let mut src = feed.get_element("src").unwrap();
        src.set("device", device);

        feed.add_element("videoconvert", "convert1");
        feed.add_element("videoscale", "scale");
        feed.add_element("videoconvert", "convert2");

        feed.link("src", "convert1");
        feed.link("convert1", "scale");
        feed.link("scale", "convert2");
        feed.add_video_shmsink("convert2");

        V4L2{feed: feed, device: String::from(device)}
    }

    pub fn play(&mut self) {
        self.feed.play();
    }

    pub fn stop(&mut self) {
        self.feed.stop();
    }
}
