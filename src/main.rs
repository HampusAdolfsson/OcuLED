extern crate fontdue;

mod output;
mod rendering;
mod screen_collection;
mod screens;

use output::RenderTarget;
use screens::drawable_component::DrawableComponent;
use std::sync::mpsc;


mod config {
    pub const ADDRESS: &str = "192.168.1.11:4435";
    pub const DISPLAY_WIDTH: usize = 128;
    pub const SCREEN_HEIGHT: usize = 64;
}

fn main() -> std::io::Result<()> {
    let clock = screens::ClockScreen{};
    let bongo_cat = rendering::Bitmap::from_png(include_bytes!("../resources/images/bongo_cat.png") as &[u8]);
    let bmp = screens::BitmapScreen{
        bitmap: bongo_cat,
        x: 0,
        y: 0,
    };
    let mut screens = screen_collection::ScreenCollection::new(vec![&clock, &bmp]);

    let (tx, rx) = mpsc::channel::<UserInput>();

    std::thread::spawn(move || {
        let mut hk = hotkey::Listener::new();
        {
            let tx = tx.clone();
            hk.register_hotkey(hotkey::modifiers::CONTROL | hotkey::modifiers::ALT | hotkey::modifiers::SHIFT,
                'O' as u32,
                move || { tx.send(UserInput::NextScreen).unwrap(); }).unwrap();
        }
        {
            let tx = tx.clone();
            hk.register_hotkey(hotkey::modifiers::CONTROL | hotkey::modifiers::ALT | hotkey::modifiers::SHIFT,
                'I' as u32,
                move || { tx.send(UserInput::PrevScreen).unwrap(); }).unwrap();
        }
        hk.listen();
    });

    let mut canvas = rendering::Canvas::new(config::DISPLAY_WIDTH, config::SCREEN_HEIGHT);
    let font = include_bytes!("../resources/fonts/Roboto-Bold.ttf") as &[u8];
    canvas.set_font(font);
    let output = output::UdpOutput{ address: config::ADDRESS };

    loop {
        canvas.clear();
        screens.draw_to(&mut canvas);
        output.render_bitmap((&canvas.bitmap).into())?;
        let event = rx.recv_timeout(std::time::Duration::from_millis(1000));
        match event {
            Ok(UserInput::NextScreen) => screens.next(),
            Ok(UserInput::PrevScreen) => screens.previous(),
            Err(_) => (),
        }
    }
}

enum UserInput {
    NextScreen,
    PrevScreen,
}
