extern crate gst;

use feeds::Feed;

pub struct CG {
    name: String,
    control_pipe_name: String,
    width: u32,
    height: u32,
    framerate: String,
    pipeline: gst::Pipeline,
}

impl CG {
    pub fn new(name: &str, width: u32, height: u32, framerate: &str) -> CG {
        let pipeline = gst::Pipeline::new(name).unwrap();
        let control_pipe_name = format!("/tmp/{}-control-pipe", name);

        let mut feed = CG{name: String::from(name),
                          control_pipe_name: control_pipe_name,
                          width: width,
                          height: height,
                          framerate: String::from(framerate),
                          pipeline: pipeline};

        feed.add_element("filesrc", "src");
        feed.add_element("pngdec", "pngdec");
        feed.add_element("videoconvert", "videoconvert");
        feed.add_element("videoscale", "videoscale");
        feed.add_element("imagefreeze", "imagefreeze");
        feed.link("src", "pngdec");
        feed.link("pngdec", "videoconvert");
        feed.link("videoconvert", "videoscale");
        feed.link("videoscale", "imagefreeze");
        feed.add_video_shmsink("imagefreeze");

        let mut src = feed.get_element("src").unwrap();
        src.set("location", "media/nature.png");

        feed
    }
}

impl Feed for CG {
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
