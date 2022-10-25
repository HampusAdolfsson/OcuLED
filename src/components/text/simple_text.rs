use crate::rendering;

use super::super::{ Size, Widget, Bounds };
use super::TextWidget;

/// A static string of text anchored in its top-left corner and taking up just enough space to fit its contents.
pub struct SimpleTextWidget<'a> {
    text: String,
    font_size: f32,
    font: &'a fontdue::Font,
}

impl<'a> SimpleTextWidget<'a> {
    pub fn new(text: String, font: &'a fontdue::Font, font_size: f32) -> Self {
        SimpleTextWidget {
            text,
            font_size,
            font,
        }
    }
}

impl<'a> TextWidget<u32, u32> for SimpleTextWidget<'a> {
    fn set_text(&mut self, text: String ) -> bool {
        if self.text.ne(&text) {
            self.text = text;
            return true;
        }
        false
    }
}

impl<'a> Widget<u32, u32> for SimpleTextWidget<'a> {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        // TODO: figure out how to expose base height
        // TODO: use size, avoid drawing out of bounds
        // TODO: cache the rendered text for performance reasons (text values likely won't change every frame)
        let text_rendered = rendering::Bitmap::from_text(&self.text, self.font_size, self.font);
        canvas.draw_bitmap(bounds.pos.x, bounds.pos.y, &text_rendered);
    }

    fn size(&self) -> Size<u32, u32> {
        // TODO: allow controlling which height to use (total height or base height)
        // TODO: cache this between calls to set_text
        let size = rendering::measure_text(&self.text, self.font, self.font_size);
        Size {
            width: size.width.try_into().unwrap(),
            height: size.height.try_into().unwrap(),
        }
    }
}
