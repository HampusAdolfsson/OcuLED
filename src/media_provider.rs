use std::rc::Rc;
use std::sync::Mutex;

use windows::Media::Control;

/// Provides information about media being played on the computer.
pub trait MediaProvider {
    /// The name of the current track, if available.
    fn track_name(&self) -> Option<String>;
    /// Checks whether the current track name matches `other`.
    ///
    /// This is useful to check whether the value has changed,
    /// and avoid unnecessarily allocating [String]s.
    fn track_name_is(&self, other: &Option<String>) -> bool;
    /// The artist of the current track, if available.
    fn artist(&self) -> Option<String>;
    /// Checks whether the current artist matches `other`.
    ///
    /// This is useful to check whether the value has changed,
    /// and avoid unnecessarily allocating [String]s.
    fn artist_is(&self, other: &Option<String>) -> bool;
    /// Whether media is currently playing ([false]) or paused ([true]).
    fn paused(&self) -> Option<bool>;
}

/// A media provider that must be explicitly told to update its data.
pub struct PollingMediaProvider {
    manager: Control::GlobalSystemMediaTransportControlsSessionManager,
    track_name: Option<String>,
    artist: Option<String>,
    paused: Option<bool>,
}

impl PollingMediaProvider {
    pub fn new() -> Self {
        Self {
            manager: Control::GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap(),
            track_name: None,
            artist: None,
            paused: None,
        }
    }

    pub fn update_media_info(&mut self) {
        let selected_session: std::option::Option<Control::GlobalSystemMediaTransportControlsSession> = self.manager.GetCurrentSession().ok();
        if let Some(session) = selected_session {
            self.paused = session.GetPlaybackInfo().unwrap().PlaybackStatus().map(
                |status| status != Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
            ).ok();
            self.track_name = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap().Title().map(|str| str.to_string()).ok();
            self.artist = session.TryGetMediaPropertiesAsync().unwrap().get().unwrap().Artist().map(|str| str.to_string()).ok();
        } else {
            self.track_name = None;
            self.artist = None;
            self.paused = None;
        }
    }
}

impl MediaProvider for PollingMediaProvider {
    fn track_name(&self) -> Option<String> {
        self.track_name.clone()
    }
    fn track_name_is(&self, other: &Option<String>) -> bool {
        self.track_name.eq(other)
    }
    fn artist(&self) -> Option<String> {
        self.artist.clone()
    }
    fn artist_is(&self, other: &Option<String>) -> bool {
        self.artist.eq(other)
    }
    fn paused(&self) -> Option<bool> {
        self.paused
    }
}
impl<M: MediaProvider> MediaProvider for Rc<Mutex<M>> {
    fn track_name(&self) -> Option<String> {
        let provider = self.lock().unwrap();
        provider.track_name()
    }
    fn track_name_is(&self, other: &Option<String>) -> bool {
        let provider = self.lock().unwrap();
        provider.track_name_is(other)
    }
    fn artist(&self) -> Option<String> {
        let provider = self.lock().unwrap();
        provider.artist()
    }
    fn artist_is(&self, other: &Option<String>) -> bool {
        let provider = self.lock().unwrap();
        provider.artist_is(other)
    }
    fn paused(&self) -> Option<bool> {
        let provider = self.lock().unwrap();
        provider.paused()
    }
}
