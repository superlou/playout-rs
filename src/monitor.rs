use gst;
use feeds::{VideoConfig, Feed};

pub struct Monitor {
    pub width: u32,
    pub height: u32,
    pub framerate: String,
    control_pipe_name: String,
    pipeline: gst::Pipeline,
}

impl Monitor {
    pub fn new(socket_path: &str, config: VideoConfig) -> Monitor {
        let pipeline = gst::Pipeline::new("pipeline").unwrap();

        let mut monitor = Monitor{width: config.width,
                                  height: config.height,
                                  framerate: config.framerate.clone(),
                                  control_pipe_name: String::from(socket_path),
                                  pipeline: pipeline};

        monitor.add_element("queue", "queue1");
        monitor.add_element("videoconvert", "videoconvert");
        monitor.add_element("queue", "queue2");
        monitor.add_element("autovideosink", "videosink");

        monitor.link("queue1", "videoconvert");
        monitor.link("videoconvert", "queue2");
        monitor.link("queue2", "videosink");

        monitor.add_video_shmsrc("queue1");

        monitor
    }
}

impl Feed for Monitor {
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
