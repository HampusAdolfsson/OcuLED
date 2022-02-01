extern crate fontdue;

mod output;
mod rendering;
mod display_controller;
mod screens;

use std::sync::mpsc;


mod config {
    pub const ADDRESS: &str = "192.168.1.11:4435";
    pub const DISPLAY_WIDTH: usize = 128;
    pub const DISPLAY_HEIGHT: usize = 64;
}

fn main() -> std::io::Result<()> {
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

    let clock = screens::ClockScreen{};
    let bmp = screens::BitmapScreen{
        bitmap: rendering::Bitmap::from_png(include_bytes!("../resources/images/bongo_cat.png")),
        x: 0,
        y: 0,
    };
    let media = screens::media::MediaControls::new();

    let mut display_controller = display_controller::DisplayController::new(config::DISPLAY_WIDTH, config::DISPLAY_HEIGHT, vec![&clock, &bmp, &media]);
    let output = output::UdpOutput{ address: config::ADDRESS };

    loop {
        display_controller.draw_to(&output)?;

        let event = rx.recv_timeout(std::time::Duration::from_millis(1000));
        match event {
            Ok(UserInput::NextScreen) => display_controller.next_screen(),
            Ok(UserInput::PrevScreen) => display_controller.previous_screen(),
            Err(_) => (),
        }
    }
}

enum UserInput {
    NextScreen,
    PrevScreen,
}
