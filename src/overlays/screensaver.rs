use rand::prelude::Distribution;

use crate::components::{VideoWidget, EmptyBounds, Bounds, Widget, Drawable};
use crate::rendering::Video;

pub struct ScreensaverOverlay {
    videos: Vec<VideoWidget>,
    current_video: Option<usize>,
}

impl ScreensaverOverlay {
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
            current_video: None,
        }
    }

    pub fn show(&mut self) {
        let distribution = rand::distributions::Uniform::from(0..self.videos.len());
        self.current_video = Some(distribution.sample(&mut rand::thread_rng()));
    }
    pub fn hide(&mut self) {
        self.current_video = None;
    }
}

impl Drawable for ScreensaverOverlay {
    fn draw(&mut self, canvas: &mut crate::rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        if let Some(current_video) = self.current_video {
            canvas.clear();
            let active_video = self.videos.get_mut(current_video).unwrap();
            let video_bounds = EmptyBounds::new().with_size(active_video.size()).center_in(&bounds);
            active_video.draw(canvas, bounds, elapsed);
        }
    }
}
