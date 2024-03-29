#![windows_subsystem = "windows"]
extern crate fontdue;
#[macro_use]
extern crate lazy_static;

mod output;
mod screens;
mod performance_monitor;
mod overlays;
mod fonts;
mod media_provider;

mod network_receiver;

use std::rc::Rc;
use std::sync::{mpsc, Mutex};

use graphics::components::{self, Drawable, Bounds};
use graphics::rendering;
use output::RenderTarget;

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

    let media_provider = Rc::new(Mutex::new(media_provider::PollingMediaProvider::new()));
    media_provider.lock().unwrap().update_media_info();

    let mut clock = screens::clock::ClockScreen::new();
    let mut media = screens::media::MediaScreen::new(Rc::clone(&media_provider));
    let stats_monitor = performance_monitor::PerformanceMonitor::new();
    let mut perf_mem = screens::performance::PerformanceWithMemoryScreen::new(stats_monitor.statistics());
    let mut perf_temp = screens::performance::PerformanceWithTemperatureScreen::new(stats_monitor.statistics());
    let mut stickfight = screens::stickfight::StickFightScreen::new(config::DISPLAY_WIDTH, config::DISPLAY_HEIGHT).unwrap();

    let mut media_overlay = overlays::MediaOverlay::new(Rc::clone(&media_provider));
    let mut screensaver = overlays::ScreensaverOverlay::new();

    let mut screens = screens::ScreenCollection::new(
        vec![
            &mut clock,
            &mut media,
            &mut perf_mem,
            &mut perf_temp,
            &mut stickfight
        ],
    );
    let mut output = output::UdpOutput{ address: config::ADDRESS, previous: rendering::BinaryBitmap{ width: 0, height: 0, buffer: Vec::new() } };

    let mut last_time = std::time::Instant::now();
    let mut canvas = rendering::Bitmap::new(config::DISPLAY_WIDTH, config::DISPLAY_HEIGHT);
    let canvas_bounds = Bounds::cover_bitmap(&canvas);
    loop {
        let elapsed = last_time.elapsed();
        last_time = std::time::Instant::now();

        media_provider.lock().unwrap().update_media_info();
        canvas.clear();
        let drawables: [&mut dyn Drawable; 3] = [&mut screens, &mut screensaver, &mut media_overlay];
        for drawable in drawables {
            drawable.draw(&mut canvas, canvas_bounds, &elapsed);
        }
        if let Err(e) = output.render_bitmap((&canvas).into()) {
            println!("Failed to send bitmap: {:?}", e);
        }

        let event = rx.recv_timeout(std::time::Duration::from_millis(50));
        match event {
            Ok(UserInput::NextScreen) => screens.next_screen(),
            Ok(UserInput::PrevScreen) => screens.previous_screen(),
            Ok(UserInput::ScreensaverOn) => screensaver.show(),
            Ok(UserInput::ScreensaverOff) => screensaver.hide(),
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
