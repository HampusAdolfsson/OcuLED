use image::{ImageResult, RgbImage, AnimationDecoder, RgbaImage};

use crate::components::{VideoWidget, Widget, Bounds, EmptyBounds};
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

impl Screen for StickFightScreen {
    fn on_mount(&mut self) {
        self.widget.reset();
    }

    fn draw_to(&mut self, canvas: &mut crate::rendering::Bitmap, elapsed: &std::time::Duration) {
        let canvas_bounds = Bounds::cover_bitmap(&canvas);
        let bounds = EmptyBounds::new().with_size(self.widget.size()).center_in(&canvas_bounds);
        self.widget.draw(canvas, bounds, elapsed);
    }
}
