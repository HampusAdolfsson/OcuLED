pub mod drawable_component;
use chrono::Local;
use drawable_component::DrawableComponent;
use crate::rendering;

pub struct ClockScreen;

impl DrawableComponent for ClockScreen {
    fn draw_to(&self, canvas: &mut rendering::Canvas) {
        let now = Local::now();
        let clock_text = now.format("%H:%M").to_string();
        canvas.draw_text(
            canvas.bitmap.width as i32 / 2,
            canvas.bitmap.height as i32 / 2,
            &clock_text,
            36.0,
            rendering::HorizontalAlignment::Center,
            rendering::VerticalAlignment::CenterBase);
    }
}

pub struct BitmapScreen {
    pub bitmap: rendering::Bitmap,
    pub x: i32,
    pub y: i32,
}

impl DrawableComponent for BitmapScreen {
    fn draw_to(&self, canvas: &mut rendering::Canvas) {
        canvas.bitmap.draw_bitmap(self.x, self.y, &self.bitmap);
    }
}