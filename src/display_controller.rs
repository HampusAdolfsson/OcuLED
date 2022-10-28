use windows::Media::PlaybackRateChangeRequestedEventArgs;

use crate::components::Widget;
use crate::rendering;
use super::screens::Screen;
use super::output;

pub struct DisplayController<'a, S: Screen> {
    screens: Vec<&'a mut dyn Screen>,
    active_screen: usize,
    canvas: rendering::Bitmap,
    indicator: ScreenIndicator,
    screensaver: S,
    screensaver_active: bool,
}

impl<'a, S: Screen> DisplayController<'a, S> {
    pub fn new(width: usize, height: usize, mut screens: Vec<&'a mut dyn Screen>, screensaver: S) -> Self {
        let mut canvas = rendering::Bitmap::new(width, height);
        let num_screens = screens.len();
        screens[0].on_mount();
        DisplayController{
            screens: screens,
            active_screen: 0,
            canvas: canvas,
            indicator: ScreenIndicator::new(num_screens),
            screensaver,
            screensaver_active: false,
        }
    }

    pub fn next_screen(&mut self) {
        if self.screensaver_active { return; }
        let prev = self.active_screen;
        self.active_screen = (self.active_screen + 1) % self.screens.len();
        self.screens[self.active_screen].on_mount();
        self.indicator.show(prev, false);
    }
    pub fn previous_screen(&mut self) {
        if self.screensaver_active { return; }
        let prev = self.active_screen;
        self.active_screen = (self.active_screen + self.screens.len() - 1) % self.screens.len();
        self.screens[self.active_screen].on_mount();
        self.indicator.show(prev, true);
    }
    pub fn set_screensaver_active(&mut self, active: bool) {
        if self.screensaver_active == active { return; }
        self.screensaver_active = active;
        if active {
            self.screensaver.on_mount();
        } else {
            self.screens[self.active_screen].on_mount();
        }
    }

    pub fn draw_to(&mut self, target: &mut dyn output::RenderTarget, elapsed: &std::time::Duration) -> std::io::Result<()> {
        self.update(elapsed);

        self.canvas.clear();

        // let canvas_bounds = crate::components::Bounds::cover_bitmap(&self.canvas);
        // let bounds = crate::components::EmptyBounds::new().with_size(self.gif.size()).center_in(&canvas_bounds);
        // self.gif.draw(&mut self.canvas, bounds, elapsed);
        if self.screensaver_active {
            self.screensaver.draw_to(&mut self.canvas, elapsed);
        } else {
            let active_screen = &mut self.screens[self.active_screen];
            active_screen.draw_to(&mut self.canvas, elapsed);
            if self.indicator.should_draw() {
                self.indicator.draw_to(&mut self.canvas);
            }
        }

        target.render_bitmap((&self.canvas).into())
    }

    fn update(&mut self, elapsed: &std::time::Duration) {
        self.indicator.tick(elapsed);
    }
}

struct ScreenIndicator {
    elapsed: std::time::Duration,
    move_duration: std::time::Duration,
    wait_duration: std::time::Duration,
    collapse_duration: std::time::Duration,
    num_screens: usize,
    from: usize,
    to_left: bool,
}

impl ScreenIndicator {
    fn draw_to(&self, canvas: &mut rendering::Bitmap) {
        let size = self.num_screens;
        let rect_width = canvas.width as i32 / size as i32;
        let from = rect_width * self.from as i32;
        let target = from + if self.to_left { -rect_width } else { rect_width };

        let move_progress = (self.elapsed.as_millis() as f32 / self.move_duration.as_millis() as f32).min(1.0);
        let move_progress_smoothed = (std::f32::consts::FRAC_PI_2 * move_progress).sin();
        let at = move_progress_smoothed * target as f32 + (1.0-move_progress_smoothed) * from as f32;

        let collapse_progress = ((self.elapsed.as_millis() as i32 - self.move_duration.as_millis() as i32 - self.wait_duration.as_millis() as i32) as f32 / self.collapse_duration.as_millis() as f32).min(1.0).max(0.0);
        let adjusted_width = ((rect_width-4) as f32 * (1.0 - collapse_progress)) as i32;

        canvas.draw_rect(at as i32 + (rect_width - adjusted_width) / 2, canvas.height as i32 - 1, adjusted_width as usize, 1);

        // check underflow
        if at < 0.0 {
            canvas.draw_rect(canvas.width as i32 + at as i32 + (rect_width - adjusted_width) / 2, canvas.height as i32 - 1, adjusted_width as usize, 1);
        }
        // check overflow
        let overflow = at as i32 + rect_width - canvas.width as i32;
        if overflow > 0 {
            canvas.draw_rect(overflow - rect_width + (rect_width - adjusted_width) / 2, canvas.height as i32 - 1, adjusted_width as usize, 1);
        }
    }
}

impl ScreenIndicator {
    pub fn new(num_screens: usize) -> Self {
        ScreenIndicator {
            elapsed: std::time::Duration::MAX,
            move_duration: std::time::Duration::from_millis(600),
            wait_duration: std::time::Duration::from_millis(200),
            collapse_duration: std::time::Duration::from_millis(400),
            num_screens: num_screens,
            from: 0,
            to_left: true,
        }
    }
    pub fn should_draw(&self) -> bool {
        self.elapsed < self.move_duration + self.wait_duration + self.collapse_duration
    }
    pub fn tick(&mut self, elapsed: &std::time::Duration) -> bool {
        if self.should_draw() {
            self.elapsed += *elapsed;
            return true
        }
        false
    }

    pub fn show(&mut self, from: usize, to_left: bool) {
        self.elapsed = std::time::Duration::ZERO;
        self.from = from;
        self.to_left = to_left;
    }
}