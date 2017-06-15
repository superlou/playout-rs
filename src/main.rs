extern crate gst;

mod feed;
mod video_test_feed;
use video_test_feed::VideoTestFeed;

fn main() {
    gst::init();

    let mut feed = VideoTestFeed::new("feed1", 1280, 720, "30/1");
    feed.play();
    feed.stop();
}
