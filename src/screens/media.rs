use crate::media_provider::MediaProvider;
use crate::rendering;
use crate::components::{ScrollingTextWidget, Alignment, BitmapWidget, TextWidget, Bounds, EmptyBounds, Widget, Drawable};

use super::{fonts, Screen};

pub struct MediaScreen<M: MediaProvider> {
    provider: M,
    icon: BitmapWidget,
    pause_icon: BitmapWidget,
    track_name: ScrollingTextWidget<'static>,
    last_track_name: Option<String>,
    artist: ScrollingTextWidget<'static>,
    last_artist: Option<String>,
}

const FONT_SIZE: f32 = 16.0;
const FONT: &[u8] = include_bytes!("../../resources/fonts/Pixellari.ttf");
const SCROLL_WAIT: std::time::Duration = std::time::Duration::from_secs(5); // time to wait between scrolling
const SCROLL_WAIT_END: std::time::Duration = std::time::Duration::from_secs(2); // time to wait at scroll end

impl<M: MediaProvider> MediaScreen<M> {
    pub fn new(provider: M) -> Self {
        let icon = BitmapWidget::new(rendering::Bitmap::from_png(include_bytes!["../../resources/images/musical_note.png"]));
        let pause_icon = BitmapWidget::new(rendering::Bitmap::from_png(include_bytes!["../../resources/images/pause.png"]));
        MediaScreen {
            provider,
            icon,
            pause_icon,
            track_name: ScrollingTextWidget::new("".to_string(), &fonts::PIXELLARI, FONT_SIZE, Alignment::Center, SCROLL_WAIT, SCROLL_WAIT_END),
            last_track_name: None,
            artist: ScrollingTextWidget::new("".to_string(), &fonts::PIXELOID, 9.0, Alignment::Center, SCROLL_WAIT, SCROLL_WAIT_END),
            last_artist: None,
        }
    }
}

impl<M: MediaProvider> Drawable for MediaScreen<M> {
    fn draw(&mut self, canvas: &mut crate::rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        if !self.provider.track_name_is(&None) && !self.provider.artist_is(&None) && self.provider.paused().is_some() {
            if !self.provider.track_name_is(&self.last_track_name) {
                self.last_track_name = self.provider.track_name();
                self.track_name.set_text(self.last_track_name.as_ref().unwrap());
            }
            if !self.provider.artist_is(&self.last_artist) {
                self.last_artist = self.provider.artist();
                self.artist.set_text(self.last_artist.as_ref().unwrap());
            }
            let icon = if !self.provider.paused().unwrap() { &mut self.icon } else { &mut self.pause_icon };
            let icon_bounds = EmptyBounds::new()
                .with_size(icon.size())
                .with_y(8).center_hor_in(&bounds);
            icon.draw(canvas, icon_bounds, elapsed);

            let title_bounds = EmptyBounds::new()
                .with_width(bounds.size.width).with_height(self.track_name.size().height)
                .with_x(0)
                .center_ver_in(&bounds).move_y(5);
            self.track_name.draw(canvas, title_bounds, elapsed);

            let artist_bounds = EmptyBounds::new()
                .with_width(bounds.size.width).with_height(self.track_name.size().height)
                .with_x(0).below(&title_bounds).move_y(3);
            self.artist.draw(canvas, artist_bounds, elapsed);
        } else {
            self.track_name.set_text("No media playing");
            let bounds = EmptyBounds::new()
                .with_width(bounds.size.width).with_height(self.track_name.size().height)
                .with_x(0).center_ver_in(&bounds);
            self.track_name.draw(canvas, bounds, elapsed);
        };
    }
}

impl<M: MediaProvider> Screen for MediaScreen<M> {
    fn on_mount(&mut self) {}
}
