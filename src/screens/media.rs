use windows::Media::Control;
use crate::rendering;

pub struct MediaControls {
    manager: Control::GlobalSystemMediaTransportControlsSessionManager,
    icon: rendering::Bitmap,
    pause_icon: rendering::Bitmap,
    title: Option<Box<dyn MediaText>>,
    artist: Option<Box<dyn MediaText>>,
}

const FONT_SIZE: f32 = 16.0;
const FONT: &[u8] = include_bytes!("../../resources/fonts/Pixellari.ttf");
const SCROLL_SPEED: f32 = 40.0; // text scrolling speed in pixels per second
const SCROLL_WAIT: std::time::Duration = std::time::Duration::from_secs(5); // time to wait between scrolling
const SCROLL_WAIT_END: std::time::Duration = std::time::Duration::from_secs(2); // time to wait at scroll end

impl MediaControls {
    pub fn new() -> Self {
        let mngr = Control::GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap();
        let icon = rendering::Bitmap::from_png(include_bytes!["../../resources/images/musical_note.png"]);
        let pause_icon = rendering::Bitmap::from_png(include_bytes!["../../resources/images/pause.png"]);
        MediaControls{ manager: mngr, icon: icon, pause_icon: pause_icon, artist: None, title: None }
    }

    fn create_text(&mut self, text: String, canvas: &rendering::Canvas) -> Box<dyn MediaText> {
        let metrics = canvas.measure_text(&text, FONT_SIZE);
        let mut text_canvas = rendering::Canvas::new(metrics.width, metrics.height);
        text_canvas.set_font(canvas.get_font().clone());
        text_canvas.draw_text(0, 0, &text, FONT_SIZE, rendering::HorizontalAlignment::Left, rendering::VerticalAlignment::Top);

        if metrics.width < canvas.bitmap.width {
            Box::new(CenteredText{
                text: text,
                rendered_text: text_canvas.bitmap,
            })
        } else {
            Box::new(ScrollingText::new(text, text_canvas.bitmap))
        }
    }
}

impl super::Screen for MediaControls {
    fn on_mount(&mut self, canvas: &mut rendering::Canvas) {
        canvas.set_font(fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap());
    }

    fn draw_to(&mut self, canvas: &mut crate::rendering::Canvas, elapsed: &std::time::Duration) {
        let selected_session: std::option::Option<Control::GlobalSystemMediaTransportControlsSession> = self.manager.GetCurrentSession().ok();
        let mut selected_status: std::option::Option<Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus> = None;
        if selected_session.is_some() {
            selected_status = selected_session.as_ref().unwrap().GetPlaybackInfo().unwrap().PlaybackStatus().ok();
        }
        match selected_session {
            Some(session) => {
                let icon = if selected_status.unwrap() == Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing { &self.icon } else { &self.pause_icon };
                canvas.bitmap.draw_bitmap((canvas.bitmap.width - self.icon.width) as i32 / 2, 0, icon);

                let title = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap().Title().unwrap().to_string();
                let song_changed = match self.title.as_ref() {
                    Some(song_component) => song_component.get_text() != &title,
                    None => true,
                };
                if song_changed {
                    self.title = Some(self.create_text(title, canvas));
                }
                self.title.as_mut().unwrap().draw(canvas, canvas.bitmap.height as i32 / 2 - 5, elapsed);

                let artist = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap().Artist().unwrap().to_string();
                let artist_changed = match self.artist.as_ref() {
                    Some(artist_component) => artist_component.get_text() != (&artist),
                    None => true,
                };
                if artist_changed {
                    self.artist = Some(self.create_text(artist, canvas));
                }
                self.artist.as_mut().unwrap().draw(canvas, canvas.bitmap.height as i32 - 16, elapsed);
            },
            None => {
                canvas.draw_text(
                    canvas.bitmap.width as i32 / 2,
                    canvas.bitmap.height as i32 / 2,
                    "No media playing",
                    16.0,
                    rendering::HorizontalAlignment::Center,
                    rendering::VerticalAlignment::CenterBase);
            },
        };
    }
}

trait MediaText {
    fn draw(&mut self, canvas: &mut rendering::Canvas, y: i32, elapsed: &std::time::Duration);
    fn get_text(&self) -> &str;
}

struct CenteredText {
    text: std::string::String,
    rendered_text: rendering::Bitmap,
}
impl MediaText for CenteredText {
    fn draw(&mut self, canvas: &mut rendering::Canvas, y: i32, _: &std::time::Duration) {
        canvas.bitmap.draw_bitmap((canvas.bitmap.width - self.rendered_text.width) as i32 / 2, y, &self.rendered_text);
    }
    fn get_text<'a>(&'a self) -> &'a str {
        &self.text
    }
}

enum ScrollingState {
    WaitingStart(std::time::Duration),
    Moving(f32),
    WaitingEnd(std::time::Duration),
}
struct ScrollingText {
    text: std::string::String,
    rendered_text: rendering::Bitmap,
    state: ScrollingState,
    // x_pos: f32,
    // wait_time: std::time::Duration,
}
impl ScrollingText {
    fn new(text: std::string::String, rendered_text: rendering::Bitmap) -> Self {
        ScrollingText{
            text: text,
            rendered_text: rendered_text,
            state: ScrollingState::WaitingStart(SCROLL_WAIT),
        }
    }
}
impl MediaText for ScrollingText {
    fn draw(&mut self, canvas: &mut rendering::Canvas, y: i32, elapsed: &std::time::Duration) {
        match self.state {
            // technically we should care about the time overflow when changing state, but...
            ScrollingState::WaitingStart(remaining) => {
                canvas.bitmap.draw_bitmap(0, y, &self.rendered_text);
                let new_remaining = remaining.saturating_sub(*elapsed);
                if new_remaining > std::time::Duration::ZERO {
                    self.state = ScrollingState::WaitingStart(new_remaining);
                } else {
                    self.state = ScrollingState::Moving(0.0);
                }
            },
            ScrollingState::WaitingEnd(remaining) => {
                canvas.bitmap.draw_bitmap((canvas.bitmap.width - self.rendered_text.width) as i32, y, &self.rendered_text);
                let new_remaining = remaining.saturating_sub(*elapsed);
                if new_remaining > std::time::Duration::ZERO {
                    self.state = ScrollingState::WaitingEnd(new_remaining);
                } else {
                    self.state = ScrollingState::WaitingStart(SCROLL_WAIT);
                }
            },
            ScrollingState::Moving(mut x_pos) => {
                x_pos -= SCROLL_SPEED * (elapsed.as_millis() as f32 / 1000.0);
                canvas.bitmap.draw_bitmap(x_pos as i32, y, &self.rendered_text);
                if (x_pos as i32 + self.rendered_text.width as i32) < canvas.bitmap.width as i32 {
                    self.state = ScrollingState::WaitingEnd(SCROLL_WAIT_END);
                } else {
                    self.state = ScrollingState::Moving(x_pos);
                }
            }
        }
    }
    fn get_text<'a>(&'a self) -> &'a str {
        &self.text
    }
}