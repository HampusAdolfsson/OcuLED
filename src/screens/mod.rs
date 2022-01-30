pub mod drawable_component;
pub mod media;

use chrono::Local;
use drawable_component::DrawableComponent;
use crate::rendering;

pub trait Screen: drawable_component::DrawableComponent {
    /**
     * Called when this screen is switched to, and will be drawn soon
     */
    fn on_mount(&self, canvas: &mut rendering::Canvas);
}

pub struct ClockScreen;

impl Screen for ClockScreen {
    fn on_mount(&self, canvas: &mut rendering::Canvas) {
        canvas.set_font(include_bytes!("../../resources/fonts/Roboto-Bold.ttf"));
    }
}
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

impl Screen for BitmapScreen {
    fn on_mount(&self, _: &mut rendering::Canvas) { }
}
impl DrawableComponent for BitmapScreen {
    fn draw_to(&self, canvas: &mut rendering::Canvas) {
        canvas.bitmap.draw_bitmap(self.x, self.y, &self.bitmap);
    }
}