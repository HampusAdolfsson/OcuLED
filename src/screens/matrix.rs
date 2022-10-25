use super::Screen;
use crate::rendering;

pub struct MatrixScreen {

}

impl MatrixScreen {
    pub fn new() -> Self {
        MatrixScreen { }
    }
}

impl Screen for MatrixScreen {
    fn on_mount(&mut self) {

    }

    fn draw_to(&mut self, canvas: &mut rendering::Bitmap, elapsed: &std::time::Duration) {

    }

}

struct Letter {
    pub x: i32,
    pub y: i32,
    pub last_changed: std::time::Instant,
}