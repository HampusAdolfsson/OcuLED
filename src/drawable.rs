use chrono::Local;
use super::rendering;

pub trait DrawableComponent {
    fn draw_to(&self, canvas: &mut rendering::Canvas);
}

pub struct ClockScreen;

impl DrawableComponent for ClockScreen {
    fn draw_to(&self, canvas: &mut rendering::Canvas) {
        let now = Local::now();
        let clock_text = now.format("%H:%M").to_string();
        canvas.draw_text(
            canvas.bitmap.width as i32 / 2,
            canvas.bitmap.height as i32 / 2,
            &clock_text,
            32.0,
            rendering::HorizontalAlignment::Center,
            rendering::VerticalAlignment::CenterBase);
    }
}