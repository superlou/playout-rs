use gst;
use feeds::{VideoConfig, Feed};

pub struct Monitor {
    pub width: u32,
    pub height: u32,
    pub framerate: String,
    control_pipe_name: String,
    pipeline: gst::Pipeline,
}

extern {
    fn gst_video_overlay_set_window_handle(
        GstXOverlay: *mut gst::ffi::Struct__GstElement,
        xwindow_id: u32
    );
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
        monitor.add_element("xvimagesink", "videosink");
        // monitor.set_window_xid(148897799);

        monitor.link("queue1", "videoconvert");
        monitor.link("videoconvert", "queue2");
        monitor.link("queue2", "videosink");

        monitor.add_video_shmsrc("queue1");

        monitor
    }

    fn set_window_xid(&mut self, xid: u32) {
        let mut videosink = self.get_element("videosink").unwrap();
        unsafe {
            gst_video_overlay_set_window_handle(videosink.gst_element_mut(), xid);
        }
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
