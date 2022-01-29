use super::screens::drawable_component::DrawableComponent;

/**
 * An ordered set of screens that can be displayed on an output.
 * Multiplexes between the screens, so that only on screen is active and displaying at any one time.
 */
pub struct ScreenCollection<'a> {
    screens: Vec<&'a dyn DrawableComponent>,
    active_screen: usize,
}

impl<'a> ScreenCollection<'a> {
    pub fn new(screens: Vec<&'a dyn DrawableComponent>) -> ScreenCollection<'a> {
        assert_ne!(screens.len(), 0);
        ScreenCollection {
            screens: screens,
            active_screen: 0,
        }
    }

    pub fn next(&mut self) {
        self.active_screen = (self.active_screen + 1) % self.screens.len();
    }
    pub fn previous(&mut self) {
        self.active_screen = (self.active_screen + self.screens.len() - 1) % self.screens.len();
    }
}

impl<'a> DrawableComponent for ScreenCollection<'a> {
    fn draw_to(&self, canvas: &mut super::rendering::Canvas) {
        let active_screen = self.screens[self.active_screen];
        active_screen.draw_to(canvas);
    }
}