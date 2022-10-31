mod performance_with_mem;
mod performance_with_temp;

pub use performance_with_mem::PerformanceWithMemoryScreen;
pub use performance_with_temp::PerformanceWithTemperatureScreen;

use crate::components::{Widget, Bounds, Size, Drawable};
use crate::rendering;

const SMOOTHING_FACTOR: f32 = 0.2;

/// A horizontal bar displaying a value in the range [0.0, 1.0]
struct BarWidget {
    value: f32,
    target: f32,
}

impl BarWidget {
    pub fn new() -> Self {
        BarWidget { value: 0.0, target: 0.0 }
    }

    /// Sets the value to be displayed
    pub fn set_value(&mut self, mut value: f32) {
        value = value.clamp(0.0, 1.0);
        self.value = value;
        self.target = value;
    }
    /// Sets the target value to be displayed. The actual value will approach this over time.
    pub fn set_value_smoothed(&mut self, mut value: f32) {
        value = value.clamp(0.0, 1.0);
        self.target = value;
    }
}

impl Drawable for BarWidget {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        self.value = update_smooth_value(self.value, self.target, elapsed);
        draw_bar_limits(canvas, bounds);
        canvas.draw_rect_with_slits(bounds.pos.x + 1, bounds.pos.y + 2, ((bounds.size.width as f32 - 2.0) * self.value) as usize, bounds.size.height as usize - 4, 4);
    }
}
impl Widget<(), ()> for BarWidget {
    fn size(&self) -> Size<(), ()> {
        Size { width: (), height: () }
    }
}

struct DoubleBarWidget {
    value_1: f32,
    target_1: f32,
    value_2: f32,
    target_2: f32,
}
impl DoubleBarWidget {
    pub fn new() -> Self {
        DoubleBarWidget { value_1: 0.0, target_1: 0.0, value_2: 0.0, target_2: 0.0 }
    }

    /// Sets the value to be displayed
    pub fn set_values(&mut self, mut value_1: f32, mut value_2: f32) {
        value_1 = value_1.clamp(0.0, 1.0);
        value_2 = value_2.clamp(0.0, 1.0);
        self.value_1 = value_1;
        self.value_2 = value_2;
        self.target_1 = value_1;
        self.target_2 = value_2;
    }
    /// Sets the target value to be displayed. The actual value will approach this over time.
    pub fn set_values_smoothed(&mut self, mut value_1: f32, mut value_2: f32) {
        value_1 = value_1.clamp(0.0, 1.0);
        value_2 = value_2.clamp(0.0, 1.0);
        self.target_1 = value_1;
        self.target_2 = value_2;
    }
}

impl Drawable for DoubleBarWidget {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        self.value_1 = update_smooth_value(self.value_1, self.target_1, elapsed);
        self.value_2 = update_smooth_value(self.value_2, self.target_2, elapsed);
        draw_bar_limits(canvas, bounds);
        canvas.draw_rect_with_slits(bounds.pos.x + 1, bounds.pos.y + 2, ((bounds.size.width as f32 - 2.0) * self.value_1) as usize, (bounds.size.height as usize - 4) / 2, 4);
        canvas.draw_rect(bounds.pos.x + 1, bounds.pos.y + 2 + (bounds.size.height as i32 - 4) / 2, ((bounds.size.width as f32 - 2.0) * self.value_2) as usize, (bounds.size.height as usize - 4) / 2);
    }
}
impl Widget<(), ()> for DoubleBarWidget {
    fn size(&self) -> Size<(), ()> {
        Size { width: (), height: () }
    }
}

fn draw_bar_limits(canvas: &mut rendering::Bitmap, bounds: Bounds) {
    assert!(bounds.size.height >= 4);
    assert!(bounds.size.width >= 2);

    canvas.draw_rect(bounds.pos.x, bounds.pos.y, 4, 1);
    canvas.draw_rect(bounds.pos.x, bounds.bottom() - 1, 4, 1);
    canvas.draw_rect(bounds.pos.x, bounds.pos.y, 1, bounds.size.height as usize);
    canvas.draw_rect(bounds.right() - 5, bounds.pos.y, 4, 1);
    canvas.draw_rect(bounds.right() - 5, bounds.bottom() - 1, 4, 1);
    canvas.draw_rect(bounds.right() - 1, bounds.pos.y, 1, bounds.size.height as usize);
}

fn update_smooth_value(curr_val: f32, target: f32, elapsed: &std::time::Duration) -> f32 {
    let progress = (elapsed.as_secs_f32() / SMOOTHING_FACTOR).clamp(0.0, 1.0);
    curr_val * (1.0 - progress) + target * progress
}