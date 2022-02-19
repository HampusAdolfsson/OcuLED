use super::Screen;
use crate::rendering;
use crate::performance_monitor;
use std::sync::{Arc, Mutex};

/**
 * Displays three computer performance bars (cpu usage, memory usage and combined gpu/vram usage)
 */
pub struct PerformanceScreen {
    stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>,
    cpu: f32,
    mem: f32,
    vram: f32,
    gpu: f32,
}

const TEXT_WIDTH: i32 = 45;
const BAR_HEIGHT: usize = 14;
const FONT_SIZE: f32 = 14.0;
const SMOOTHING_AMOUNT: f32 = 0.75;

impl PerformanceScreen {
    pub fn new(stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>) -> Self {
        PerformanceScreen {
            stats: stats,
            cpu: 0.0,
            mem: 0.0,
            vram: 0.0,
            gpu: 0.0,
        }
    }
}

impl Screen for PerformanceScreen {
    fn on_mount(&mut self, canvas: &mut rendering::Canvas) {
        canvas.set_font_from_bytes(include_bytes!("../../resources/fonts/Roboto-Bold.ttf"));
        // Gives a cool effect with the smoothing
        self.cpu = 0.0;
        self.mem = 0.0;
        self.gpu = 0.0;
        self.vram = 0.0;
    }

    fn draw_to(&mut self, canvas: &mut rendering::Canvas, _: &std::time::Duration) {
        {
            // this is framerate-dependent, but I'm too lazy do to it right :')
            let stats = self.stats.lock().unwrap();
            self.cpu = self.cpu * SMOOTHING_AMOUNT + stats.cpu_usage * (1.0 - SMOOTHING_AMOUNT);
            self.mem = self.mem * SMOOTHING_AMOUNT + stats.memory_usage * (1.0 - SMOOTHING_AMOUNT);
            self.vram = self.vram * SMOOTHING_AMOUNT + stats.vram_usage * (1.0 - SMOOTHING_AMOUNT);
            self.gpu = self.gpu * SMOOTHING_AMOUNT + stats.gpu_usage * (1.0 - SMOOTHING_AMOUNT);
        };

        let bar_width = canvas.bitmap.width - TEXT_WIDTH as usize;
        canvas.draw_text(0, BAR_HEIGHT as i32 / 2, "CPU", FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::CenterBase);
        self.draw_bar(canvas, TEXT_WIDTH, 0, BAR_HEIGHT, bar_width, self.cpu);

        canvas.draw_text(0, canvas.bitmap.height as i32 / 2, "MEM", FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::CenterBase);
        self.draw_bar(canvas, TEXT_WIDTH, (canvas.bitmap.height / 2 - BAR_HEIGHT / 2) as i32, BAR_HEIGHT, bar_width, self.mem);

        canvas.draw_text(0, (canvas.bitmap.height - BAR_HEIGHT / 2) as i32, "GPU", FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::CenterBase);
        self.draw_bar_double(canvas, TEXT_WIDTH, (canvas.bitmap.height - BAR_HEIGHT) as i32, BAR_HEIGHT, bar_width, self.gpu, self.vram);

    }
}

impl PerformanceScreen {
    fn draw_bar(&self, canvas: &mut rendering::Canvas, x: i32, y: i32, height: usize, width: usize, fill: f32) {
        self.draw_bar_limits(canvas, x, y, height, width);
        canvas.bitmap.draw_rect_with_slits(x + 1, y + 2, ((width as f32 - 2.0) * fill) as usize, height - 4, 4);
    }

    fn draw_bar_double(&self, canvas: &mut rendering::Canvas, x: i32, y: i32, height: usize, width: usize, fill_upper: f32, fill_lower: f32) {
        self.draw_bar_limits(canvas, x, y, height, width);
        // fill bar
        canvas.bitmap.draw_rect_with_slits(x + 1, y + 2, ((width as f32 - 2.0) * fill_upper) as usize, (height - 4) / 2, 4);
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