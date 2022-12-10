use super::super::Screen;
use crate::fonts;
use super::{BarWidget,DoubleBarWidget};
use crate::components::{Bounds, Drawable};
use crate::components::EmptyBounds;
use crate::components::SimpleTextWidget;
use crate::components::Widget;
use crate::rendering;
use crate::performance_monitor;
use std::sync::{Arc, Mutex};

/**
 * Displays three computer performance bars (cpu usage, memory usage and combined gpu/vram usage)
 */
pub struct PerformanceWithMemoryScreen {
    stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>,
    cpu_widgets: (SimpleTextWidget<'static>, DoubleBarWidget),
    mem_widgets: (SimpleTextWidget<'static>, BarWidget),
    gpu_widgets: (SimpleTextWidget<'static>, DoubleBarWidget),
}

const BAR_HEIGHT: u32 = 10;
const TEXT_PADDING: u32 = 7;
const VERTICAL_PADDING: u32 = 8;
const FONT_SIZE: f32 = 9.0;

impl PerformanceWithMemoryScreen {
    pub fn new(stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>) -> Self {
        PerformanceWithMemoryScreen {
            stats: stats,
            cpu_widgets: (SimpleTextWidget::new("CPU".to_string(), &fonts::PIXELOID, FONT_SIZE), DoubleBarWidget::new()),
            mem_widgets: (SimpleTextWidget::new("MEM".to_string(), &fonts::PIXELOID, FONT_SIZE), BarWidget::new()),
            gpu_widgets: (SimpleTextWidget::new("GPU".to_string(), &fonts::PIXELOID, FONT_SIZE), DoubleBarWidget::new()),
        }
    }
}

impl Drawable for PerformanceWithMemoryScreen {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        {
            let stats = self.stats.lock().unwrap();
            self.cpu_widgets.1.set_values_smoothed(stats.cpu_usage_group_1, stats.cpu_usage_group_2);
            self.mem_widgets.1.set_value_smoothed(stats.memory_usage);
            self.gpu_widgets.1.set_values_smoothed(stats.gpu_usage, stats.vram_usage);
        };


        let canvas_bounds = bounds
            .move_y(VERTICAL_PADDING as i32).with_height(bounds.size.height as u32 - 2 * VERTICAL_PADDING);
        let text_width = TEXT_PADDING + self.cpu_widgets.0.size().width.max(self.mem_widgets.0.size().width.max(self.gpu_widgets.0.size().width));
        let text_bounds = EmptyBounds::new().with_width(text_width).with_x(0);
        let bar_bounds = EmptyBounds::new()
            .with_height(BAR_HEIGHT)
            .between_hor(text_bounds.right(), canvas_bounds.size.width as i32);

        {
            let cpu_bar_bounds = bar_bounds.align_top(&canvas_bounds);
            self.cpu_widgets.1.draw(canvas, cpu_bar_bounds, elapsed);
            let cpu_text_bounds = text_bounds.with_height(self.cpu_widgets.0.size().height).center_ver_in(&cpu_bar_bounds);
            self.cpu_widgets.0.draw(canvas, cpu_text_bounds, elapsed);
        }
        {
            let mem_bar_bounds = bar_bounds.center_ver_in(&canvas_bounds);
            self.mem_widgets.1.draw(canvas, mem_bar_bounds, elapsed);
            let mem_text_bounds = text_bounds.with_height(self.mem_widgets.0.size().height).center_ver_in(&mem_bar_bounds);
            self.mem_widgets.0.draw(canvas, mem_text_bounds, elapsed);
        }
        {
            let gpu_bar_bounds = bar_bounds.align_bottom(&canvas_bounds);
            self.gpu_widgets.1.draw(canvas, gpu_bar_bounds, elapsed);
            let gpu_text_bounds = text_bounds.with_height(self.gpu_widgets.0.size().height).center_ver_in(&gpu_bar_bounds);
            self.gpu_widgets.0.draw(canvas, gpu_text_bounds, elapsed);
        }
        // canvas.draw_text(0, (canvas.bitmap.height - BAR_HEIGHT / 2) as i32, "GPU", FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::CenterBase);
        // self.draw_bar_double(canvas, TEXT_WIDTH, (canvas.height - BAR_HEIGHT) as i32, BAR_HEIGHT, bar_width, self.gpu, self.vram);

    }
}

impl Screen for PerformanceWithMemoryScreen {
    fn on_mount(&mut self) {
        // Gives a cool effect with the smoothing
        self.cpu_widgets.1.set_values(0.0, 0.0);
        self.mem_widgets.1.set_value(0.0);
        self.gpu_widgets.1.set_values(0.0, 0.0);
    }
}
