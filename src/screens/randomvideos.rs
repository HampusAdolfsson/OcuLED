use image::ImageResult;
use rand::prelude::Distribution;

use crate::components::{VideoWidget, Widget, EmptyBounds, Bounds};
use crate::rendering::Video;

use super::Screen;


pub struct RandomVideosScreen {
    videos: Vec<VideoWidget>,
    current_video: usize,
}

impl RandomVideosScreen {
    pub fn new() -> Self {
        let raw_bytes: Vec<(&[u8], bool)> = vec![
            (include_bytes!("../../resources/gifs/fishy.gif"), false),
            (include_bytes!("../../resources/gifs/ghost.gif"), false),
            (include_bytes!("../../resources/gifs/infinity.gif"), true),
            (include_bytes!("../../resources/gifs/kitty.gif"), true),
            (include_bytes!("../../resources/gifs/legday.gif"), false),
            (include_bytes!("../../resources/gifs/planets.gif"), false),
        ];
        let videos = raw_bytes.into_iter().map(|(bytes, invert)| Video::from_gif(bytes, invert).unwrap());
        let widgets = videos.map(|vid| VideoWidget::new(vid, 20.0));
        Self {
            videos: widgets.collect(),
            current_video: 0,
        }
    }
}

impl Screen for RandomVideosScreen {
    fn on_mount(&mut self) {
        let distribution = rand::distributions::Uniform::from(0..self.videos.len());
        self.current_video = distribution.sample(&mut rand::thread_rng());
    }

    fn draw_to(&mut self, canvas: &mut crate::rendering::Bitmap, elapsed: &std::time::Duration) {
        let active_video = self.videos.get_mut(self.current_video).unwrap();

        let canvas_bounds = Bounds::cover_bitmap(&canvas);
        let bounds = EmptyBounds::new().with_size(active_video.size()).center_in(&canvas_bounds);
        active_video.draw(canvas, bounds, elapsed);
    }
}
