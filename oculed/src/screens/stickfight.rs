use image::{ImageResult, AnimationDecoder, RgbaImage};

use crate::components::{VideoWidget, Widget, Bounds, EmptyBounds, Drawable};
use crate::rendering::Video;

use super::Screen;


pub struct StickFightScreen {
    widget: VideoWidget,
}

impl StickFightScreen {
    pub fn new(width: usize, height: usize) -> ImageResult<Self> {
        let cursor = std::io::Cursor::new(include_bytes!("../../resources/gifs/stickfight.gif"));
        let decoder = image::codecs::gif::GifDecoder::new(cursor).unwrap();
        let frames = decoder.into_frames().collect_frames()?;
        let images = frames.into_iter().map(|mut f| {
            image::imageops::invert(f.buffer_mut());
            image::imageops::resize(f.buffer(), width as u32, height as u32, image::imageops::FilterType::Nearest)
        }).collect::<Vec<RgbaImage>>();

        let video = Video::from_images(&images);
        Ok(Self {
            widget: VideoWidget::new(video, 15.0)
        })
    }
}

impl Drawable for StickFightScreen {
    fn draw(&mut self, canvas: &mut crate::rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        let video_bounds = EmptyBounds::new().with_size(self.widget.size()).center_in(&bounds);
        self.widget.draw(canvas, video_bounds, elapsed);
    }
}

impl Screen for StickFightScreen {
    fn on_mount(&mut self) {
        self.widget.reset();
    }
}
