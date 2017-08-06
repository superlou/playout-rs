use gst;
use std::path::PathBuf;
use std::process::{Command};
use uuid::Uuid;
use feeds::Feed;
use feeds::VideoConfig;

pub struct CG {
    name: String,
    control_pipe_name: String,
    width: u32,
    height: u32,
    framerate: String,
    pipeline: gst::Pipeline,
    slides: Vec<Slide>
}

pub struct Slide {
    svg_path: PathBuf,
    png_path: PathBuf,
}

impl CG {
    pub fn new(name: &str, config: &VideoConfig) -> CG {
        let pipeline = gst::Pipeline::new(name).unwrap();
        let control_pipe_name = format!("/tmp/{}-control-pipe", name);

        let mut feed = CG{name: String::from(name),
                          control_pipe_name: control_pipe_name,
                          width: config.width,
                          height: config.height,
                          framerate: config.framerate.clone(),
                          pipeline: pipeline,
                          slides: Vec::new()};

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

    pub fn create_slide(&mut self, svg_path: PathBuf) {
        let slide = Slide::new(svg_path, self.width, self.height);
        self.slides.push(slide);
    }

    pub fn select_slide(&mut self, slide_id: usize) {
        let mut src = self.get_element("src").unwrap();

        let slide = &mut self.slides[slide_id];
        let png_path = slide.png_path.to_str().unwrap();

        src.set("location", png_path);
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

impl Slide {
    pub fn new(svg_path: PathBuf, width: u32, height: u32) -> Slide {
        let mut png_path = PathBuf::from("/tmp/slides");
        png_path.push(Uuid::new_v4().to_string());
        png_path.set_extension("png");

        println!("Test: {:?}", png_path);

        let _ = Command::new("inkscape").arg(svg_path.to_str().unwrap())
                                        .arg(format!("--export-png={}", png_path.to_str().unwrap()))
                                        .arg(format!("--export-width={}", width))
                                        .arg(format!("--export-height={}", height))
                                        .output();

        Slide{svg_path: svg_path,
              png_path: png_path}
    }
}
