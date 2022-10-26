use windows::Media::Control;
use crate::rendering;
use crate::components::{Widget, ScrollingTextWidget, Alignment, TextWidget, Bounds, EmptyBounds, BitmapWidget};

use super::fonts;

pub struct MediaControls {
    manager: Control::GlobalSystemMediaTransportControlsSessionManager,
    icon: BitmapWidget,
    pause_icon: BitmapWidget,
    title: ScrollingTextWidget<'static>,
    artist: ScrollingTextWidget<'static>,
}

const FONT_SIZE: f32 = 16.0;
const FONT: &[u8] = include_bytes!("../../resources/fonts/Pixellari.ttf");
const SCROLL_WAIT: std::time::Duration = std::time::Duration::from_secs(5); // time to wait between scrolling
const SCROLL_WAIT_END: std::time::Duration = std::time::Duration::from_secs(2); // time to wait at scroll end

impl MediaControls {
    pub fn new() -> Self {
        let mngr = Control::GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap();
        let icon = BitmapWidget::new(rendering::Bitmap::from_png(include_bytes!["../../resources/images/musical_note.png"]));
        let pause_icon = BitmapWidget::new(rendering::Bitmap::from_png(include_bytes!["../../resources/images/pause.png"]));
        MediaControls {
            manager: mngr,
            icon,
            pause_icon,
            artist: ScrollingTextWidget::new("".to_string(), &fonts::PIXELOID, 9.0, Alignment::Center, SCROLL_WAIT, SCROLL_WAIT_END),
            title: ScrollingTextWidget::new("".to_string(), &fonts::PIXELLARI, FONT_SIZE, Alignment::Center, SCROLL_WAIT, SCROLL_WAIT_END),
        }
    }
}

impl super::Screen for MediaControls {
    fn on_mount(&mut self) {}

    fn draw_to(&mut self, canvas: &mut crate::rendering::Bitmap, elapsed: &std::time::Duration) {
        let selected_session: std::option::Option<Control::GlobalSystemMediaTransportControlsSession> = self.manager.GetCurrentSession().ok();
        let mut selected_status: std::option::Option<Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus> = None;
        if selected_session.is_some() {
            selected_status = selected_session.as_ref().unwrap().GetPlaybackInfo().unwrap().PlaybackStatus().ok();
        }

        let canvas_bounds = Bounds::cover_bitmap(canvas);
        match selected_session {
            Some(session) => {
                let icon = if selected_status.unwrap() == Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing { &mut self.icon } else { &mut self.pause_icon };
                let icon_bounds = EmptyBounds::new()
                    .with_size(icon.size())
                    .with_y(8).center_hor_in(&canvas_bounds);
                icon.draw(canvas, icon_bounds, elapsed);

                let title = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap().Title().unwrap().to_string();
                self.title.set_text(title);
                let title_bounds = EmptyBounds::new()
                    .with_width(canvas.width as u32).with_height(self.title.size().height)
                    .with_x(0)
                    .center_ver_in(&canvas_bounds).move_y(5);
                self.title.draw(canvas, title_bounds, elapsed);

                let artist = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap().Artist().unwrap().to_string();
                self.artist.set_text(artist);
                let artist_bounds = EmptyBounds::new()
                    .with_width(canvas.width as u32).with_height(self.title.size().height)
                    .with_x(0).below(&title_bounds).move_y(3);
                self.artist.draw(canvas, artist_bounds, elapsed);

            },
            None => {
                self.title.set_text("No media playing".to_string());
                let bounds = EmptyBounds::new()
                    .with_width(canvas.width as u32).with_height(self.title.size().height)
                    .with_x(0).center_ver_in(&canvas_bounds);
                self.title.draw(canvas, bounds, elapsed);
            },
        };
    }
}
