use graphics::components;
use graphics::rendering;

pub mod media;
pub mod performance;
pub mod stickfight;
pub mod clock;

pub trait Screen : components::Drawable {
    /// Called when this screen is switched to, and will be drawn soon
    fn on_mount(&mut self);
}

/// Holds several [Screen]s and allows cycling between them (see [ScreenCollection::next_screen]).
pub struct ScreenCollection<'a> {
    screens: Vec<&'a mut dyn Screen>,
    active_screen: usize,
    indicator: ScreenIndicator,
}

impl<'a> ScreenCollection<'a> {
    pub fn new(mut screens: Vec<&'a mut dyn Screen>) -> Self {
        let num_screens = screens.len();
        screens[0].on_mount();
        ScreenCollection{
            screens: screens,
            active_screen: 0,
            indicator: ScreenIndicator::new(num_screens),
        }
    }

    pub fn next_screen(&mut self) {
        let prev = self.active_screen;
        self.active_screen = (self.active_screen + 1) % self.screens.len();
        self.screens[self.active_screen].on_mount();
        self.indicator.show(prev, false);
    }
    pub fn previous_screen(&mut self) {
        let prev = self.active_screen;
        self.active_screen = (self.active_screen + self.screens.len() - 1) % self.screens.len();
        self.screens[self.active_screen].on_mount();
        self.indicator.show(prev, true);
    }
}

impl<'a> components::Drawable for ScreenCollection<'a> {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: components::Bounds, elapsed: &std::time::Duration) {
        let active_screen = &mut self.screens[self.active_screen];
        active_screen.draw(canvas, bounds, elapsed);
        if self.indicator.should_draw() {
            self.indicator.draw(canvas, bounds, elapsed);
        }
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

impl components::Drawable for ScreenIndicator {
    fn draw(&mut self, canvas: &mut rendering::Bitmap, bounds: components::Bounds, elapsed: &std::time::Duration) {
        self.elapsed += *elapsed;

        let size = self.num_screens;
        let rect_width = bounds.size.width as i32 / size as i32;
        let from = rect_width * self.from as i32;
        let target = from + if self.to_left { -rect_width } else { rect_width };

        let move_progress = (self.elapsed.as_millis() as f32 / self.move_duration.as_millis() as f32).min(1.0);
        let move_progress_smoothed = (std::f32::consts::FRAC_PI_2 * move_progress).sin();
        let at = move_progress_smoothed * target as f32 + (1.0-move_progress_smoothed) * from as f32;

        let collapse_progress = ((self.elapsed.as_millis() as i32 - self.move_duration.as_millis() as i32 - self.wait_duration.as_millis() as i32) as f32 / self.collapse_duration.as_millis() as f32).min(1.0).max(0.0);
        let adjusted_width = ((rect_width-4) as f32 * (1.0 - collapse_progress)) as i32;

        canvas.draw_rect(at as i32 + (rect_width - adjusted_width) / 2, bounds.size.height as i32 - 1, adjusted_width as usize, 1);

        // check underflow
        if at < 0.0 {
            canvas.draw_rect(bounds.size.width as i32 + at as i32 + (rect_width - adjusted_width) / 2, bounds.size.height as i32 - 1, adjusted_width as usize, 1);
        }
        // check overflow
        let overflow = at as i32 + rect_width - canvas.width as i32;
        if overflow > 0 {
            canvas.draw_rect(overflow - rect_width + (rect_width - adjusted_width) / 2, bounds.size.height as i32 - 1, adjusted_width as usize, 1);
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

    pub fn show(&mut self, from: usize, to_left: bool) {
        self.elapsed = std::time::Duration::ZERO;
        self.from = from;
        self.to_left = to_left;
    }
}