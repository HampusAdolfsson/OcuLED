extern crate fontdue;

mod rendering;
mod output;

use output::RenderTarget;

mod config {
    pub const ADDRESS: &str = "192.168.1.11:4435";
    pub const SCREEN_WIDTH: usize = 128;
    pub const SCREEN_HEIGHT: usize = 64;
}

fn main() -> std::io::Result<()> {
    let mut canvas = rendering::Canvas::new(config::SCREEN_WIDTH, config::SCREEN_HEIGHT);

    // Read the font data.
    let font = include_bytes!("../resources/Roboto-Regular.ttf") as &[u8];
    canvas.set_font(font);
    canvas.draw_text(0, 50, "ciog", 31.0);

    let output = output::UdpOutput{ address: config::ADDRESS };
    output.render_canvas(canvas.into())?;
    Ok(())
}
