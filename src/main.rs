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
    for x in 0..config::SCREEN_WIDTH / 2 {
        for y in 0..config::SCREEN_HEIGHT / 2 {
            canvas.buffer[128*y + x] = 0xff;
        }
    }

    let output = output::UdpOutput{ address: config::ADDRESS };
    output.render_canvas(canvas.into())?;
    Ok(())
}
