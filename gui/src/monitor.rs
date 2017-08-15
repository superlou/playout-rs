// todo Remove the duplicate feed structure
use std::fs;
use gst;
use gst::Element;

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
    pub fn new(socket_path: &str, config: &VideoConfig) -> Monitor {
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

        monitor.link("queue1", "videoconvert");
        monitor.link("videoconvert", "queue2");
        monitor.link("queue2", "videosink");

        monitor.add_video_shmsrc("queue1");

        monitor
    }

    pub fn set_window_xid(&mut self, xid: u32) {
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

#[derive(Clone)]
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

    fn add_video_shmsrc(&mut self, first_element_name: &str) {
        let mut first_element = self.get_pipeline().get_by_name(first_element_name).unwrap();
        let (width, height) = self.get_dimensions();
        let framerate = self.get_framerate().clone();

        self.add_element("shmsrc", "src");
        let mut src = self.get_element("src").unwrap();
        src.set("socket-path", &self.get_control_pipe_name() as &str);
        src.set("do-timestamp", true);
        src.set("is-live", true);

        let mixer_format = "video/x-raw, format=BGRA, pixel-aspect-ratio=1/1, interlace-mode=progressive";
        let caps_string = format!("{}, width={}, height={}, framerate={}",
                                  mixer_format,
                                  width,
                                  height,
                                  framerate);

        let caps = gst::Caps::from_string(&caps_string).unwrap();
        src.link_filtered(&mut first_element, &caps);
    }

    fn play(&mut self) {
        self.get_pipeline().play();
    }

    fn stop(&mut self) {
        self.get_pipeline().set_null_state();
    }
}
