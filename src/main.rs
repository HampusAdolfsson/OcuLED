#![windows_subsystem = "windows"]
extern crate fontdue;

mod output;
mod rendering;
mod display_controller;
mod screens;
mod performance_monitor;

use std::sync::mpsc;


mod config {
    pub const ADDRESS: &str = "192.168.1.41:4435";
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
        {
            let tx = tx.clone();
            hk.register_hotkey(hotkey::modifiers::CONTROL | hotkey::modifiers::ALT | hotkey::modifiers::SHIFT,
                'P' as u32,
                move || { tx.send(UserInput::Quit).unwrap(); }).unwrap();
        }
        hk.listen();
    });

    let mut clock = screens::ClockScreen{};
    let mut media = screens::media::MediaControls::new();

    let stats_monitor = performance_monitor::PerformanceMonitor::new();
    let mut perf = screens::performance::PerformanceScreen::new(stats_monitor.statistics());
    let mut perf_temperature = screens::performance_with_temp::PerformanceWithTemperatureScreen::new(stats_monitor.statistics());

    let mut display_controller = display_controller::DisplayController::new(config::DISPLAY_WIDTH, config::DISPLAY_HEIGHT, vec![&mut clock, &mut media, &mut perf, &mut perf_temperature]);
    let output = output::UdpOutput{ address: config::ADDRESS };

    let mut last_time = std::time::Instant::now();
    loop {
        let elapsed = last_time.elapsed();
        last_time = std::time::Instant::now();
        display_controller.tick(elapsed);
        display_controller.draw_to(&output, &elapsed)?;

        let event = rx.recv_timeout(std::time::Duration::from_millis(25));
        match event {
            Ok(UserInput::NextScreen) => display_controller.next_screen(),
            Ok(UserInput::PrevScreen) => display_controller.previous_screen(),
            Ok(UserInput::Quit) => break Ok(()),
            Err(_) => {},
        }
    }
}

enum UserInput {
    NextScreen,
    PrevScreen,
    Quit,
}
