use gst;
use feeds::Feed;
use feeds::VideoConfig;

pub struct V4L2 {
    name: String,
    control_pipe_name: String,
    width: u32,
    height: u32,
    framerate: String,
    pipeline: gst::Pipeline,
    device: String,
}

impl V4L2 {
    pub fn new(name: &str, config: &VideoConfig, device: &str) -> V4L2 {
        let pipeline = gst::Pipeline::new(name).unwrap();
        let control_pipe_name = format!("/tmp/{}-control-pipe", name);

        let mut feed = V4L2{device: String::from(device),
                            name: String::from(name),
                            control_pipe_name: control_pipe_name,
                            width: config.width,
                            height: config.height,
                            framerate: config.framerate.clone(),
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
        feed
    }
}

impl Feed for V4L2 {
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
