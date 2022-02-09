use windows::Media::Control;
use crate::rendering;

pub struct MediaControls {
    manager: Control::GlobalSystemMediaTransportControlsSessionManager,
    icon: rendering::Bitmap,
    pause_icon: rendering::Bitmap,
}


impl MediaControls {
    pub fn new() -> Self {
        let mngr = Control::GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap();
        let icon = rendering::Bitmap::from_png(include_bytes!["../../resources/images/musical_note.png"]);
        let pause_icon = rendering::Bitmap::from_png(include_bytes!["../../resources/images/pause.png"]);
        MediaControls{ manager: mngr, icon: icon, pause_icon: pause_icon }
    }
}

impl super::Screen for MediaControls {
    fn on_mount(&mut self, canvas: &mut rendering::Canvas) {
        canvas.set_font(include_bytes!("../../resources/fonts/Pixellari.ttf"));
        // canvas.set_font(include_bytes!("../../resources/fonts/Roboto-Medium.ttf"));
    }

    fn draw_to(&mut self, canvas: &mut crate::rendering::Canvas) {
        let sessions = self.manager.GetSessions().unwrap();

        let mut selected_session: std::option::Option<Control::GlobalSystemMediaTransportControlsSession> = None;
        let mut selected_status: std::option::Option<Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus> = None;

        for i in 0..sessions.Size().unwrap() {
            let session = sessions.GetAt(i).unwrap();
            let status = session.GetPlaybackInfo().unwrap().PlaybackStatus().unwrap();
            if status == Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing && selected_status != Some(Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing) {
                selected_session = Some(session);
                selected_status = Some(status);
                continue;
            }
            if selected_session == None {
                selected_session = Some(session);
                selected_status = Some(status);
            }
        }
        if selected_session == None {
            selected_session = self.manager.GetCurrentSession().ok();
            if selected_session != None {
                selected_status = selected_session.as_ref().unwrap().GetPlaybackInfo().unwrap().PlaybackStatus().ok();
            }
        }
        match selected_session {
            Some(session) => {
                let title = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap().Title().unwrap().to_string();
                let artist = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap().Artist().unwrap().to_string();
                let icon = if selected_status.unwrap() == Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing { &self.icon } else { &self.pause_icon };
                canvas.bitmap.draw_bitmap((canvas.bitmap.width - self.icon.width) as i32 / 2, 0, icon);
                canvas.draw_text(
                    canvas.bitmap.width as i32 / 2,
                    canvas.bitmap.height as i32 / 2,
                    &title,
                    16.0,
                    rendering::HorizontalAlignment::Center,
                    rendering::VerticalAlignment::CenterBase);
                canvas.draw_text(
                    canvas.bitmap.width as i32 / 2,
                    canvas.bitmap.height as i32 - 6,
                    &artist,
                    16.0,
                    rendering::HorizontalAlignment::Center,
                    rendering::VerticalAlignment::Bottom);
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
        // canvas.draw_text();
    }
}