use crate::rendering::Video;

use super::{Widget, Size, Bounds, Drawable};


pub struct VideoWidget {
    video: Video,
    time_per_frame: std::time::Duration,
    time: std::time::Duration,
}

impl VideoWidget {
    pub fn new(video: Video, fps: f32) -> Self {
        VideoWidget {
            video,
            time_per_frame: std::time::Duration::from_secs_f32(1.0/fps),
            time: std::time::Duration::ZERO,
        }
    }

    pub fn reset(&mut self) {
        self.video.reset();
    }
}

impl Drawable for VideoWidget {
    fn draw(&mut self, canvas: &mut crate::rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        self.time += *elapsed;
        while self.time > self.time_per_frame {
            self.time -= self.time_per_frame;
            self.video.advance();
        }
        let frame = self.video.current_frame();
        canvas.draw_bitmap(bounds.pos.x, bounds.pos.y, frame);
    }
}

impl Widget<u32, u32> for VideoWidget {
    fn size(&self) -> Size<u32, u32> {
        let frame = self.video.current_frame();
        Size {
            width: frame.width as u32,
            height: frame.height as u32,
        }
    }
}
