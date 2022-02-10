use super::Screen;
use crate::rendering;
use crate::performance_monitor;
use std::sync::{Arc, Mutex};

/**
 * Displays two computer performance bars (cpu usage and combined gpu/vram usage), as well as cpu and gpu temperatures
 */
pub struct PerformanceWithTemperatureScreen {
    stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>,
    cpu: f32,
    vram: f32,
    gpu: f32,
    gpu_temp: u32,
    cpu_temp: f32,
    cpu_icon: rendering::Bitmap,
    gpu_icon: rendering::Bitmap,
}

const TEXT_WIDTH: i32 = 45;
const BAR_HEIGHT: usize = 10;
const FONT_SIZE: f32 = 14.0;
const SMOOTHING_AMOUNT: f32 = 0.5;
const SEPARATOR_MARGIN: i32 = 6;

impl PerformanceWithTemperatureScreen {
    pub fn new(stats: Arc<Mutex<performance_monitor::PerformanceStatistics>>) -> Self {
        PerformanceWithTemperatureScreen {
            stats: stats,
            cpu: 0.0,
            vram: 0.0,
            gpu: 0.0,
            gpu_temp: 0,
            cpu_temp: 0.0,
            cpu_icon: rendering::Bitmap::from_png(include_bytes!("../../resources/images/cpu.png")),
            gpu_icon: rendering::Bitmap::from_png(include_bytes!("../../resources/images/gpu.png")),
        }
    }
}

impl Screen for PerformanceWithTemperatureScreen {
    fn on_mount(&mut self, canvas: &mut rendering::Canvas) {
        canvas.set_font(include_bytes!("../../resources/fonts/Roboto-Bold.ttf"));
        // Gives a cool effect with the smoothing
        self.cpu = 0.0;
        self.gpu = 0.0;
        self.vram = 0.0;
    }

    fn draw_to(&mut self, canvas: &mut rendering::Canvas) {
        {
            // this is framerate-dependent, but I'm too lazy do to it right :')
            let stats = self.stats.lock().unwrap();
            self.cpu = self.cpu * SMOOTHING_AMOUNT + stats.cpu_usage * (1.0 - SMOOTHING_AMOUNT);
            self.vram = self.vram * SMOOTHING_AMOUNT + stats.vram_usage * (1.0 - SMOOTHING_AMOUNT);
            self.gpu = self.gpu * SMOOTHING_AMOUNT + stats.gpu_usage * (1.0 - SMOOTHING_AMOUNT);
            self.gpu_temp = stats.gpu_temperature;
            self.cpu_temp = stats.cpu_temperature;
        };
        let bar_width = canvas.bitmap.width - TEXT_WIDTH as usize;
        let separator_pos = BAR_HEIGHT as i32 + SEPARATOR_MARGIN;

        canvas.bitmap.draw_bitmap((canvas.bitmap.width * 3 / 4 - self.cpu_icon.width / 2) as i32 + 3, (canvas.bitmap.height - self.cpu_icon.height) as i32 / 2, &self.cpu_icon);
        canvas.draw_text(canvas.bitmap.width as i32, BAR_HEIGHT as i32 / 2, &format!("{}°C", self.cpu_temp), FONT_SIZE, rendering::HorizontalAlignment::Right, rendering::VerticalAlignment::CenterBase);
        self.draw_bar(canvas, 0, 0, BAR_HEIGHT, bar_width, self.cpu);

        canvas.bitmap.draw_rect_with_slits(0, separator_pos, canvas.bitmap.width / 2, 1, 2);
        canvas.bitmap.draw_rect_with_slits(canvas.bitmap.width as i32 / 2, canvas.bitmap.height as i32 - separator_pos, canvas.bitmap.width / 2, 1, 2);
        canvas.bitmap.draw_rect_with_slits(canvas.bitmap.width as i32 / 2, separator_pos, 1, canvas.bitmap.height - 2*separator_pos as usize, 2);

        canvas.bitmap.draw_bitmap((canvas.bitmap.width / 4 - self.gpu_icon.width / 2) as i32, (canvas.bitmap.height - self.gpu_icon.height) as i32 / 2, &self.gpu_icon);
        canvas.draw_text(0, (canvas.bitmap.height - BAR_HEIGHT / 2) as i32, &format!("{}°C", self.gpu_temp), FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::CenterBase);
        self.draw_bar_double(canvas, TEXT_WIDTH, (canvas.bitmap.height - BAR_HEIGHT) as i32, BAR_HEIGHT, bar_width, self.gpu, self.vram);

    }
}

impl PerformanceWithTemperatureScreen {
    fn draw_bar(&self, canvas: &mut rendering::Canvas, x: i32, y: i32, height: usize, width: usize, fill: f32) {
        self.draw_bar_limits(canvas, x, y, height, width);
        canvas.bitmap.draw_rect_with_slits(x + 1, y + 2, ((width as f32 - 2.0) * fill) as usize, height - 4, 4);
    }

    fn draw_bar_double(&self, canvas: &mut rendering::Canvas, x: i32, y: i32, height: usize, width: usize, fill_upper: f32, fill_lower: f32) {
        self.draw_bar_limits(canvas, x, y, height, width);
        // fill bar
        canvas.bitmap.draw_rect_with_slits(x + 1, y + 2, ((width as f32 - 2.0) * fill_upper) as usize, (height - 4) / 2, 4);
        canvas.bitmap.draw_rect_with_slits(x + 1, y + 2 + (height as i32 - 4) / 2, ((width as f32 - 2.0) * fill_lower) as usize, (height - 4) / 2, 2);
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
