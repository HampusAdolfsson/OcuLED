use super::Screen;
use crate::rendering;
use crate::performance_monitor;
use std::sync::{Arc, Mutex};

pub struct PerformanceScreen {
    pub stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>,
}

const TEXT_WIDTH: i32 = 45;
const BAR_HEIGHT: usize = 14;
const FONT_SIZE: f32 = 14.0;

impl Screen for PerformanceScreen {
    fn on_mount(&self, canvas: &mut rendering::Canvas) {
        canvas.set_font(include_bytes!("../../resources/fonts/Roboto-Bold.ttf"));
    }

    fn draw_to(&mut self, canvas: &mut rendering::Canvas) {
        let stats = self.stats.lock().unwrap();

        let bar_width = canvas.bitmap.width - TEXT_WIDTH as usize;
        canvas.draw_text(0, BAR_HEIGHT as i32 / 2, "CPU", FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::CenterBase);
        let cpu = stats.cpu_usage;
        self.draw_bar(canvas, TEXT_WIDTH, 0, BAR_HEIGHT, bar_width, cpu);

        canvas.draw_text(0, canvas.bitmap.height as i32 / 2, "MEM", FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::CenterBase);
        let mem = stats.memory_usage;
        self.draw_bar(canvas, TEXT_WIDTH, (canvas.bitmap.height / 2 - BAR_HEIGHT / 2) as i32, BAR_HEIGHT, bar_width, mem);

        canvas.draw_text(0, (canvas.bitmap.height - BAR_HEIGHT / 2) as i32, "GPU", FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::CenterBase);
        let gpu = stats.gpu_usage;
        let vram = stats.vram_usage;
        self.draw_bar_double(canvas, TEXT_WIDTH, (canvas.bitmap.height - BAR_HEIGHT) as i32, BAR_HEIGHT, bar_width, gpu, vram);

    }
}

impl PerformanceScreen {
    fn draw_bar(&self, canvas: &mut rendering::Canvas, x: i32, y: i32, height: usize, width: usize, fill: f32) {
        self.draw_bar_limits(canvas, x, y, height, width);
        canvas.bitmap.draw_rect_dotted(x + 1, y + 2, ((width as f32 - 2.0) * fill) as usize, height - 4);
    }

    fn draw_bar_double(&self, canvas: &mut rendering::Canvas, x: i32, y: i32, height: usize, width: usize, fill_upper: f32, fill_lower: f32) {
        self.draw_bar_limits(canvas, x, y, height, width);
        // fill bar
        canvas.bitmap.draw_rect_dotted(x + 1, y + 2, ((width as f32 - 2.0) * fill_upper) as usize, (height - 4) / 2);
        canvas.bitmap.draw_rect(x + 1, y + 2 + (height as i32 - 4) / 2, ((width as f32 - 2.0) * fill_lower) as usize, (height - 4) / 2);
    }

    fn draw_bar_limits(&self, canvas: &mut rendering::Canvas, x: i32, y: i32, height: usize, width: usize) {
        assert!(height >= 4);
        assert!(width >= 2);
        // limits
        canvas.bitmap.draw_rect(x, y, 4, 1);
        canvas.bitmap.draw_rect(x, y + height as i32 - 1, 4, 1);
        canvas.bitmap.draw_rect(x, y, 1, height);
        canvas.bitmap.draw_rect(x + width as i32 - 5, y, 4, 1);
        canvas.bitmap.draw_rect(x + width as i32 - 5, y + height as i32 - 1, 4, 1);
        canvas.bitmap.draw_rect(x + width as i32 - 1, y, 1, height);
    }
}