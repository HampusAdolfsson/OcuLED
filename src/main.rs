// #![windows_subsystem = "windows"]
extern crate fontdue;
#[macro_use]
extern crate lazy_static;

mod output;
mod rendering;
mod display_controller;
mod screens;
mod performance_monitor;
mod components;

mod network_receiver;

use std::sync::mpsc;


mod config {
    pub const ADDRESS: &str = "192.168.1.6:4435";
    pub const DISPLAY_WIDTH: usize = 128;
    pub const DISPLAY_HEIGHT: usize = 64;
}

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<UserInput>();

    let tx2 = tx.clone();
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
        {
            let tx = tx.clone();
            hk.register_hotkey(hotkey::modifiers::CONTROL | hotkey::modifiers::ALT | hotkey::modifiers::SHIFT,
                'P' as u32,
                move || { tx.send(UserInput::Quit).unwrap(); }).unwrap();
        }
        hk.listen();
    });

    network_receiver::start(tx2);

    let mut clock = screens::ClockScreen::new();
    let mut media = screens::media::MediaControls::new();

    let stats_monitor = performance_monitor::PerformanceMonitor::new();
    let mut perf_mem = screens::performance::PerformanceWithMemoryScreen::new(stats_monitor.statistics());
    let mut perf_temp = screens::performance::PerformanceWithTemperatureScreen::new(stats_monitor.statistics());
    let mut stickfight = screens::stickfight::StickFightScreen::new(config::DISPLAY_WIDTH, config::DISPLAY_HEIGHT).unwrap();
    let screensaver = screens::randomvideos::RandomVideosScreen::new();

    let mut display_controller = display_controller::DisplayController::new(
        config::DISPLAY_WIDTH, config::DISPLAY_HEIGHT,
        vec![
            &mut clock,
            &mut media,
            &mut perf_mem,
            &mut perf_temp,
            &mut stickfight
        ],
        screensaver,
    );
    let mut output = output::UdpOutput{ address: config::ADDRESS, previous: rendering::BinaryBitmap{ width: 0, height: 0, buffer: Vec::new() } };

    let mut last_time = std::time::Instant::now();
    loop {
        let elapsed = last_time.elapsed();
        last_time = std::time::Instant::now();
        display_controller.draw_to(&mut output, &elapsed)?;

        let event = rx.recv_timeout(std::time::Duration::from_millis(50));
        match event {
            Ok(UserInput::NextScreen) => display_controller.next_screen(),
            Ok(UserInput::PrevScreen) => display_controller.previous_screen(),
            Ok(UserInput::ScreensaverOn) => display_controller.set_screensaver_active(true),
            Ok(UserInput::ScreensaverOff) => display_controller.set_screensaver_active(false),
            Ok(UserInput::Quit) => break Ok(()),
            Err(_) => {},
        }
    }
}

pub enum UserInput {
    NextScreen,
    PrevScreen,
    ScreensaverOn,
    ScreensaverOff,
    Quit,
}
