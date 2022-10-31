use crate::components::Drawable;
use crate::rendering;

use super::super::{ Size, Widget, Bounds };
use super::TextWidget;

/// A static string of text anchored in its top-left corner and taking up just enough space to fit its contents.
pub struct SimpleTextWidget<'a> {
    text: String,
    rendered_text: rendering::Bitmap,
    font_size: f32,
    font: &'a fontdue::Font,
}

impl<'a> SimpleTextWidget<'a> {
    pub fn new(text: String, font: &'a fontdue::Font, font_size: f32) -> Self {
        let bmp = rendering::Bitmap::from_text(&text, font_size, font);
        SimpleTextWidget {
            text,
            rendered_text: bmp,
            font_size,
            font,
        }
    }
}

impl<'a> TextWidget<u32, u32> for SimpleTextWidget<'a> {
    fn set_text(&mut self, text: &str ) -> bool {
        if self.text.ne(&text) {
            self.rendered_text = rendering::Bitmap::from_text(text, self.font_size, self.font);
            return true;
        }
        false
    }
}

impl<'a> Drawable for SimpleTextWidget<'a> {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: Bounds, _elapsed: &std::time::Duration) {
        // TODO: figure out how to expose base height
        // TODO: use size, avoid drawing out of bounds
        canvas.draw_bitmap(bounds.pos.x, bounds.pos.y, &self.rendered_text);
    }
}

impl<'a> Widget<u32, u32> for SimpleTextWidget<'a> {
    fn size(&self) -> Size<u32, u32> {
        // TODO: allow controlling which height to use (total height or base height)
        Size {
            width: self.rendered_text.width.try_into().unwrap(),
            height: self.rendered_text.height.try_into().unwrap(),
        }
    }
}
