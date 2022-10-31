use crate::components::{ScrollingTextWidget, BitmapWidget, Bounds, EmptyBounds, Widget, Alignment, TextWidget, SimpleTextWidget, Drawable};
use crate::media_provider::MediaProvider;
use crate::rendering::{Bitmap, self};

const PLAY_PAUSE_DURATION: std::time::Duration = std::time::Duration::from_millis(1000);
const TRACK_INFO_DURATION: std::time::Duration = std::time::Duration::from_millis(3000);

/// Temporarily shows media information when the song changes or it is paused/unpaused.
pub struct MediaOverlay<M: MediaProvider> {
    media_provider: M,
    play_pause: PlayPauseOverlay,
    track_info: TrackInfoOverlay,
    last_track_name: Option<String>,
    last_artist: Option<String>,
    last_paused: Option<bool>,
}

impl<M: MediaProvider> MediaOverlay<M> {
    pub fn new(provider: M) -> Self {
        Self {
            media_provider: provider,
            play_pause: PlayPauseOverlay::new(PLAY_PAUSE_DURATION),
            track_info: TrackInfoOverlay::new(TRACK_INFO_DURATION),
            last_track_name: None,
            last_artist: None,
            last_paused: None,
        }
    }
}

impl<M: MediaProvider> Drawable for MediaOverlay<M> {
    fn draw(&mut self, canvas: &mut crate::rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        if !self.media_provider.track_name_is(&None) && !self.media_provider.artist_is(&None) {
            if !self.media_provider.track_name_is(&self.last_track_name) || !self.media_provider.artist_is(&self.last_artist) {
                self.track_info.show(self.media_provider.track_name().as_ref().unwrap(), self.media_provider.artist().as_ref().unwrap());
            }
        }
        if !self.media_provider.track_name_is(&self.last_track_name) {
            self.last_track_name = self.media_provider.track_name();
        }
        if !self.media_provider.artist_is(&self.last_artist) {
            self.last_artist = self.media_provider.artist();
        }

        if let Some(paused) = self.media_provider.paused() {
            if self.last_paused.map(|last_paused| paused != last_paused).unwrap_or(true) {
                self.play_pause.show(paused);
            }
        }
        self.last_paused = self.media_provider.paused();

        self.play_pause.draw(canvas, bounds, elapsed);
        self.track_info.draw(canvas, bounds, elapsed);
    }
}

struct TrackInfoOverlay {
    title: SimpleTextWidget<'static>,
    artist: SimpleTextWidget<'static>,
    icon: BitmapWidget,
    duration: std::time::Duration,
    time: std::time::Duration,
}

impl TrackInfoOverlay {
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            title: SimpleTextWidget::new("".to_string(), &crate::fonts::PIXELLARI, 16.0),
            artist: SimpleTextWidget::new("".to_string(), &crate::fonts::PIXELOID, 9.0),
            icon: BitmapWidget::new(rendering::Bitmap::from_png(include_bytes!["../../resources/images/musical_note.png"])),
            duration,
            time: std::time::Duration::ZERO,
        }
    }

    pub fn show(&mut self, title: &str, artist: &str) {
        self.title.set_text(&title);
        self.artist.set_text(&artist);
        self.time = self.duration;
    }
}

impl Drawable for TrackInfoOverlay {
    fn draw(&mut self, canvas: &mut crate::rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        self.time = self.time.saturating_sub(*elapsed);
        if self.time.is_zero() {
            return;
        }
        canvas.clear();

        let icon_bounds = EmptyBounds::new().with_size(self.icon.size()).center_ver_in(&bounds).with_x(0);
        self.icon.draw(canvas, icon_bounds, elapsed);

        let title_bounds = EmptyBounds::new()
            .with_width(bounds.size.width).with_height(self.title.size().height)
            .right_of(&icon_bounds).move_x(8)
            .with_y(0);
        let artist_bounds = EmptyBounds::new()
            .with_width(bounds.size.width).with_height(self.title.size().height)
            .right_of(&icon_bounds).below(&title_bounds)
            .move_x(8).move_y(6);

        let text_bounds = EmptyBounds::new().between_ver(title_bounds.pos.y, artist_bounds.bottom()).center_ver_in(&bounds);

        self.title.draw(canvas, title_bounds.move_y(text_bounds.pos.y), elapsed);
        self.artist.draw(canvas, artist_bounds.move_y(text_bounds.pos.y), elapsed);
    }
}

struct PlayPauseOverlay {
    duration: std::time::Duration,
    time: std::time::Duration,
    is_paused: bool,
    paused_widget: BitmapWidget,
    playing_widget: BitmapWidget,
}

impl PlayPauseOverlay {
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            duration,
            time: std::time::Duration::ZERO,
            is_paused: false,
            paused_widget: BitmapWidget::new(Bitmap::from_png_with_scale(include_bytes!("../../resources/images/pause.png"), 2.0)),
            playing_widget: BitmapWidget::new(Bitmap::from_png(include_bytes!("../../resources/images/play.png"))),
        }
    }

    pub fn show(&mut self, is_paused: bool) {
        self.is_paused = is_paused;
        self.time = self.duration;
    }
}

impl Drawable for PlayPauseOverlay {
    fn draw(&mut self, canvas: &mut crate::rendering::Bitmap, bounds: Bounds, elapsed: &std::time::Duration) {
        self.time = self.time.saturating_sub(*elapsed);
        if self.time.is_zero() {
            return;
        }

        canvas.clear();
        let widget = if self.is_paused { &mut self.paused_widget } else { &mut self.playing_widget };
        let widget_bounds = EmptyBounds::new().with_size(widget.size()).center_in(&bounds);
        widget.draw(canvas, widget_bounds, elapsed);
    }
}
