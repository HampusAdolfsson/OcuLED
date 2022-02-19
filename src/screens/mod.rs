pub mod media;
pub mod performance;
pub mod performance_with_temp;

use chrono::Local;
use crate::rendering;

pub trait Screen {
    /**
     * Called when this screen is switched to, and will be drawn soon
     */
    fn on_mount(&mut self, canvas: &mut rendering::Canvas);

    fn draw_to(&mut self, canvas: &mut rendering::Canvas, elapsed: &std::time::Duration);
}

pub struct ClockScreen;

impl Screen for ClockScreen {
    fn on_mount(&mut self, canvas: &mut rendering::Canvas) {
        canvas.set_font_from_bytes(include_bytes!("../../resources/fonts/Roboto-Bold.ttf"));
    }

    fn draw_to(&mut self, canvas: &mut rendering::Canvas, _: &std::time::Duration) {
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
