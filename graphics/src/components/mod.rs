mod text;
mod layout;
mod video;
pub use text::*;
pub use layout::*;
pub use video::*;

use crate::rendering;

pub trait Drawable {
    /// Progresses the drawable by the given time and draws it to the given bitmap.
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: layout::Bounds, elapsed: &std::time::Duration);
}

pub trait Widget<W: Copy, H: Copy> : Drawable {
    fn size(&self) -> Size<W, H>;
}

pub struct BitmapWidget {
    bitmap: rendering::Bitmap,
}

impl BitmapWidget {
    pub fn new(bitmap: rendering::Bitmap) -> Self {
        BitmapWidget { bitmap }
    }
}
impl Drawable for BitmapWidget {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: layout::Bounds, _elapsed: &std::time::Duration) {
        // TODO: replace with call that respects bounds
        canvas.draw_bitmap(bounds.pos.x, bounds.pos.y, &self.bitmap);
    }
}
impl Widget<u32, u32> for BitmapWidget {
    fn size(&self) -> Size<u32, u32> {
        Size {
            width: self.bitmap.width as u32,
            height: self.bitmap.height as u32,
        }
    }
}
