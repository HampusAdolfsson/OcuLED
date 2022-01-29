extern crate fontdue;

mod drawable;
mod output;
mod rendering;

use output::RenderTarget;
use drawable::DrawableComponent;

mod config {
    pub const ADDRESS: &str = "192.168.1.11:4435";
    pub const SCREEN_WIDTH: usize = 128;
    pub const SCREEN_HEIGHT: usize = 64;
}

fn main() -> std::io::Result<()> {
    let mut canvas = rendering::Canvas::new(config::SCREEN_WIDTH, config::SCREEN_HEIGHT);

    // Read the font data.
    let font = include_bytes!("../resources/Roboto-Bold.ttf") as &[u8];
    canvas.set_font(font);

    let output = output::UdpOutput{ address: config::ADDRESS };

    let clock = drawable::ClockScreen{};
    loop {
        canvas.clear();
        // canvas.draw_text(10, 40, "ciog", 31.0);
        // x += 3;
        clock.draw_to(&mut canvas);
        output.render_bitmap((&canvas.bitmap).into())?;
        std::thread::sleep_ms(50);
    }
}
