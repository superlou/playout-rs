extern crate gst;
use gst::Element;
use feeds;


pub struct Monitor {
    pub width: u32,
    pub height: u32,
    pub framerate: String,
    pipeline: gst::Pipeline,
}

impl Monitor {
    pub fn new(socket_path: &str, config: feeds::VideoConfig) -> Monitor {
        let pipeline = gst::Pipeline::new("pipeline").unwrap();

        let mut monitor = Monitor{width: config.width,
                                  height: config.height,
                                  framerate: config.framerate.clone(),
                                  pipeline: pipeline};

        monitor.add_element("shmsrc", "src");
        let mut src = monitor.get_element("src").unwrap();
        src.set("socket-path", socket_path);
        src.set("do-timestamp", true);
        src.set("is-live", true);

        monitor.add_element("queue", "queue1");
        monitor.add_element("videoconvert", "videoconvert");
        monitor.add_element("queue", "queue2");
        monitor.add_element("autovideosink", "videosink");

        let mixer_format = "video/x-raw, format=BGRA, pixel-aspect-ratio=1/1, interlace-mode=progressive";
        let caps_string = format!("{}, width={}, height={}, framerate={}",
                                  mixer_format,
                                  config.width,
                                  config.height,
                                  config.framerate.clone());

        let caps = gst::Caps::from_string(&caps_string).unwrap();
        let mut queue = monitor.get_element("queue1").unwrap();
        src.link_filtered(&mut queue, &caps);

        monitor.link("queue1", "videoconvert");
        monitor.link("videoconvert", "queue2");
        monitor.link("queue2", "videosink");

        monitor
    }

    fn get_pipeline(&mut self) -> &mut gst::Pipeline {
        &mut self.pipeline
    }

    fn add_element(&mut self, element_type: &str, name: &str) {
        let element = gst::Element::new(element_type, name).unwrap();
        self.get_pipeline().add(element);
    }

    fn get_element(&mut self, name: &str) -> Option<Element> {
        self.get_pipeline().get_by_name(name)
    }

    pub fn play(&mut self) {
        self.get_pipeline().play();
    }

    pub fn stop(&mut self) {
        self.get_pipeline().set_null_state();
    }

    fn link(&mut self, src_name: &str, dest_name: &str) -> bool {
        let mut src = self.get_pipeline().get_by_name(src_name).unwrap();
        let mut dest = self.get_pipeline().get_by_name(dest_name).unwrap();
        src.link(&mut dest)
    }
}
