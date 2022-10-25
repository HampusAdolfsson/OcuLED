use super::BarWidget;
use super::DoubleBarWidget;
use super::super::{fonts, Screen};
use crate::components::BitmapWidget;
use crate::components::Bounds;
use crate::components::EmptyBounds;
use crate::components::SimpleTextWidget;
use crate::components::TextWidget;
use crate::components::Widget;
use crate::rendering;
use crate::performance_monitor;
use std::sync::{Arc, Mutex};

/**
 * Displays two computer performance bars (cpu usage and combined gpu/vram usage), as well as cpu and gpu temperatures
 */
pub struct PerformanceWithTemperatureScreen {
    stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>,
    cpu_widgets: (BitmapWidget, SimpleTextWidget<'static>, BarWidget),
    gpu_widgets: (BitmapWidget, SimpleTextWidget<'static>, DoubleBarWidget),
}

const TEXT_WIDTH: u32 = 45;
const BAR_HEIGHT: u32 = 10;
const FONT_SIZE: f32 = 12.0;
const SEPARATOR_MARGIN: i32 = 6;

impl PerformanceWithTemperatureScreen {
    pub fn new(stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>) -> Self {
        PerformanceWithTemperatureScreen {
            stats,
            cpu_widgets: (
                BitmapWidget::new(rendering::Bitmap::from_png(include_bytes!("../../../resources/images/cpu.png"))),
                SimpleTextWidget::new("".to_string(), &fonts::SQHEAD, FONT_SIZE),
                BarWidget::new(),
            ),
            gpu_widgets: (
                BitmapWidget::new(rendering::Bitmap::from_png(include_bytes!("../../../resources/images/gpu.png"))),
                SimpleTextWidget::new("".to_string(), &fonts::SQHEAD, FONT_SIZE),
                DoubleBarWidget::new(),
            ),
        }
    }
}

impl Screen for PerformanceWithTemperatureScreen {
    fn on_mount(&mut self) {
        // Gives a cool effect with the smoothing
        self.cpu_widgets.2.set_value(0.0);
        self.gpu_widgets.2.set_values(0.0, 0.0);
    }

    fn draw_to(&mut self, canvas: &mut rendering::Bitmap, elapsed: &std::time::Duration) {
        {
            let stats = self.stats.lock().unwrap();
            self.cpu_widgets.2.set_value_smoothed(stats.cpu_usage);
            self.cpu_widgets.1.set_text(format!("{} C", stats.cpu_temperature));
            self.gpu_widgets.2.set_values_smoothed(stats.gpu_usage, stats.vram_usage);
            self.gpu_widgets.1.set_text(format!("{} C", stats.gpu_temperature));
        };
        let bar_width = canvas.width - TEXT_WIDTH as usize;
        let separator_pos = BAR_HEIGHT as i32 + SEPARATOR_MARGIN;

        let canvas_bounds = Bounds::cover_bitmap(canvas);

        {
            let icon_bounds = EmptyBounds::new()
                .with_size(self.cpu_widgets.0.size())
                .center_ver_in(&canvas_bounds)
                .center_hor_in(&EmptyBounds::new().between_hor(canvas.width as i32 / 2, canvas.width as i32))
                .move_x(3);
            self.cpu_widgets.0.draw(canvas, icon_bounds, elapsed);
            let text_bounds_outer = EmptyBounds::new().with_height(self.cpu_widgets.1.size().height)
                .with_width(TEXT_WIDTH).align_right(&canvas_bounds);
            let bar_bounds = EmptyBounds::new()
                .with_height(BAR_HEIGHT).with_y(0)
                .between_hor(0, text_bounds_outer.pos.x);
            self.cpu_widgets.2.draw(canvas, bar_bounds, elapsed);
            let text_bounds_inner = text_bounds_outer.center_ver_in(&bar_bounds)
                .with_width(self.cpu_widgets.1.size().width).center_hor_in(&text_bounds_outer);
            self.cpu_widgets.1.draw(canvas, text_bounds_inner, elapsed)
        }
        {
            let icon_bounds = EmptyBounds::new()
                .with_size(self.gpu_widgets.0.size())
                .center_ver_in(&canvas_bounds)
                .center_hor_in(&EmptyBounds::new().between_hor(0, canvas.width as i32 / 2));
            self.gpu_widgets.0.draw(canvas, icon_bounds, elapsed);
            let text_bounds_outer = EmptyBounds::new().with_height(self.gpu_widgets.1.size().height)
                .with_width(TEXT_WIDTH).align_left(&canvas_bounds);
            let bar_bounds = EmptyBounds::new()
                .with_height(BAR_HEIGHT).align_bottom(&canvas_bounds)
                .between_hor(text_bounds_outer.right(), canvas_bounds.right());
            self.gpu_widgets.2.draw(canvas, bar_bounds, elapsed);
            let text_bounds_inner = text_bounds_outer.center_ver_in(&bar_bounds)
                .with_width(self.gpu_widgets.1.size().width).center_hor_in(&text_bounds_outer);
            self.gpu_widgets.1.draw(canvas, text_bounds_inner, elapsed)
        }

        canvas.draw_rect_with_slits(0, separator_pos, canvas.width / 2, 1, 2);
        canvas.draw_rect_with_slits(canvas.width as i32 / 2, canvas.height as i32 - separator_pos, canvas.width / 2, 1, 2);
        canvas.draw_rect_with_slits(canvas.width as i32 / 2, separator_pos, 1, canvas.height - 2*separator_pos as usize, 2);
    }
}
