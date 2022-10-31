pub mod media;
pub mod performance;
pub mod stickfight;

use chrono::Local;
use crate::components::{Widget, TextWidget, EmptyBounds, Bounds, Drawable};
use crate::{rendering, components, fonts};

pub trait Screen : Drawable {
    /// Called when this screen is switched to, and will be drawn soon
    fn on_mount(&mut self);
}

pub struct ClockScreen<'a> {
    clock_widget: components::SimpleTextWidget<'a>,
    date_widget:  components::SimpleTextWidget<'a>,
}

impl ClockScreen<'static> {
    pub fn new() -> Self {
        ClockScreen {
            clock_widget: components::SimpleTextWidget::new("".to_string(), &fonts::ROADRAGE, 30.0),
            // clock_widget: components::SimpleTextWidget::new("".to_string(), &fonts::SYMTEXT, 30.0),
            // clock_widget: components::SimpleTextWidget::new("".to_string(), &fonts::ELFBOY, 56.0),
            // clock_widget: components::SimpleTextWidget::new("".to_string(), &fonts::ROBOTO, 36.0),
            date_widget: components::SimpleTextWidget::new("".to_string(), &fonts::SYMTEXT, 10.0),
        }
    }
    fn update(&mut self, _elapsed: &std::time::Duration) {
        let now = Local::now();
        let clock_text = now.format("%H:%M").to_string();
        self.clock_widget.set_text(&clock_text);
        // let date_text = now.format("%d %b %y").to_string();
        // self.date_widget.set_text(date_text);
    }
}

impl Drawable for ClockScreen<'static> {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        self.update(elapsed);

        {
            let clock_bounds = EmptyBounds::new()
                .with_size(self.clock_widget.size())
                .center_in(&bounds);
            self.clock_widget.draw(canvas, clock_bounds, elapsed);
        }
        // {
        //     let date_bounds = EmptyBounds::new()
        //         .with_size(self.date_widget.size())
        //         .align_bottom(&canvas_bounds).center_hor_in(&bounds);
        //     self.date_widget.draw(canvas, date_bounds, elapsed);
        // }
    }
}

impl Screen for ClockScreen<'static> {
    fn on_mount(&mut self) {}

}
