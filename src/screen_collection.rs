use super::screens::Screen;
use super::rendering;

/**
 * An ordered set of screens that can be displayed on an output.
 * Multiplexes between the screens, so that only on screen is active and displaying at any one time.
 */
pub struct ScreenCollection<'a> {
    screens: Vec<&'a dyn Screen>,
    active_screen: usize,
    pub canvas: rendering::Canvas,
}

impl<'a> ScreenCollection<'a> {
    pub fn new(screens: Vec<&'a dyn Screen>, canvas: rendering::Canvas) -> ScreenCollection<'a> {
        assert_ne!(screens.len(), 0);
        ScreenCollection {
            screens: screens,
            active_screen: 0,
            canvas: canvas,
        }
    }

    pub fn next(&mut self) {
        self.active_screen = (self.active_screen + 1) % self.screens.len();
        self.screens[self.active_screen].on_mount(&mut self.canvas);
    }
    pub fn previous(&mut self) {
        self.active_screen = (self.active_screen + self.screens.len() - 1) % self.screens.len();
        self.screens[self.active_screen].on_mount(&mut self.canvas);
    }
}

impl<'a> ScreenCollection<'a> {
    pub fn draw(&mut self) {
        self.canvas.clear();
        let active_screen = self.screens[self.active_screen];
        active_screen.draw_to(&mut self.canvas);
    }
}