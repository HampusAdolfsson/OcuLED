use crate::rendering;
use super::screens::Screen;
use super::output;

pub struct DisplayController<'a> {
    screens: Vec<&'a dyn Screen>,
    active_screen: usize,
    canvas: rendering::Canvas,
    indicator: bool,
}

impl<'a> DisplayController<'a> {
    pub fn new(width: usize, height: usize, screens: Vec<&'a dyn Screen>) -> Self {
        let mut canvas = rendering::Canvas::new(width, height);
        canvas.set_font(include_bytes!("../resources/fonts/Roboto-Bold.ttf"));
        DisplayController{
            screens: screens,
            active_screen: 0,
            canvas: canvas,
            indicator: true,
        }
    }

    pub fn next_screen(&mut self) {
        self.active_screen = (self.active_screen + 1) % self.screens.len();
        self.screens[self.active_screen].on_mount(&mut self.canvas);
        self.indicator = true;
    }
    pub fn previous_screen(&mut self) {
        self.active_screen = (self.active_screen + self.screens.len() - 1) % self.screens.len();
        self.screens[self.active_screen].on_mount(&mut self.canvas);
        self.indicator = true;
    }

    pub fn draw_to(&mut self, target: &dyn output::RenderTarget) -> std::io::Result<()> {
        self.canvas.clear();
        let active_screen = self.screens[self.active_screen];
        active_screen.draw_to(&mut self.canvas);
        if self.indicator {
            self.indicator = false;
            let size = self.screens.len();
            let rect_width = self.canvas.bitmap.width / size;
            for i in 0..size {
                if i == self.active_screen {
                    self.canvas.bitmap.draw_rect((rect_width * i) as i32, self.canvas.bitmap.height as i32 - 1, rect_width, 1);
                }
            }
        }

        target.render_bitmap((&self.canvas.bitmap).into())
    }
}