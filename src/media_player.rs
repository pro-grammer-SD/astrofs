// Media Player - Interactive playback controls for audio/video
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Clone, Debug)]
pub struct MediaPlayer {
    pub current_file: String,
    pub state: PlaybackState,
    pub position: Duration,
    pub duration: Duration,
    pub volume: f32,      // 0.0 to 1.0
    pub speed: f32,       // 0.5 to 2.0
    pub repeat_mode: RepeatMode,
    pub playlist: Vec<String>,
    pub current_index: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RepeatMode {
    None,
    One,
    All,
}

impl MediaPlayer {
    /// Create new media player with defaults
    pub fn new() -> Self {
        Self {
            current_file: String::new(),
            state: PlaybackState::Stopped,
            position: Duration::ZERO,
            duration: Duration::ZERO,
            volume: 1.0,
            speed: 1.0,
            repeat_mode: RepeatMode::None,
            playlist: Vec::new(),
            current_index: 0,
        }
    }

    /// Create new media player with a specific file and duration
    pub fn with_file(file: String, duration: Duration) -> Self {
        Self {
            current_file: file,
            state: PlaybackState::Stopped,
            position: Duration::ZERO,
            duration,
            volume: 1.0,
            speed: 1.0,
            repeat_mode: RepeatMode::None,
            playlist: Vec::new(),
            current_index: 0,
        }
    }

    /// Play the media
    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    /// Pause the media
    pub fn pause(&mut self) {
        self.state = PlaybackState::Paused;
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.position = Duration::ZERO;
    }

    /// Toggle play/pause
    pub fn toggle(&mut self) {
        match self.state {
            PlaybackState::Playing => self.pause(),
            _ => self.play(),
        }
    }

    /// Seek to a specific position
    pub fn seek(&mut self, position: Duration) {
        if position <= self.duration {
            self.position = position;
        }
    }

    /// Seek forward by amount
    pub fn seek_forward(&mut self, amount: Duration) {
        let new_pos = self.position + amount;
        if new_pos <= self.duration {
            self.position = new_pos;
        } else {
            self.position = self.duration;
        }
    }

    /// Seek backward by amount
    pub fn seek_backward(&mut self, amount: Duration) {
        if self.position >= amount {
            self.position -= amount;
        } else {
            self.position = Duration::ZERO;
        }
    }

    /// Skip to beginning
    pub fn skip_start(&mut self) {
        self.position = Duration::ZERO;
    }

    /// Skip to end
    pub fn skip_end(&mut self) {
        self.position = self.duration;
    }

    /// Set volume (0.0 to 1.0)
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Increase volume
    pub fn volume_up(&mut self) {
        self.set_volume(self.volume + 0.1);
    }

    /// Decrease volume
    pub fn volume_down(&mut self) {
        self.set_volume(self.volume - 0.1);
    }

    /// Set playback speed
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.25, 2.0);
    }

    /// Increase speed
    pub fn speed_up(&mut self) {
        let new_speed = (self.speed * 100.0 + 10.0).round() / 100.0;
        self.set_speed(new_speed);
    }

    /// Decrease speed
    pub fn speed_down(&mut self) {
        let new_speed = (self.speed * 100.0 - 10.0).round() / 100.0;
        self.set_speed(new_speed);
    }

    /// Reset speed to normal
    pub fn speed_normal(&mut self) {
        self.speed = 1.0;
    }

    /// Cycle repeat mode
    pub fn cycle_repeat(&mut self) {
        self.repeat_mode = match self.repeat_mode {
            RepeatMode::None => RepeatMode::One,
            RepeatMode::One => RepeatMode::All,
            RepeatMode::All => RepeatMode::None,
        };
    }

    /// Get progress as percentage (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.duration.as_millis() > 0 {
            self.position.as_millis() as f32 / self.duration.as_millis() as f32
        } else {
            0.0
        }
    }

    /// Format current position as string (MM:SS)
    pub fn position_string(&self) -> String {
        let secs = self.position.as_secs();
        let mins = secs / 60;
        let secs = secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    /// Format duration as string (MM:SS)
    pub fn duration_string(&self) -> String {
        let secs = self.duration.as_secs();
        let mins = secs / 60;
        let secs = secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    /// Get status bar text
    pub fn status_bar(&self) -> String {
        let pos = self.position_string();
        let dur = self.duration_string();
        let state = match self.state {
            PlaybackState::Playing => "▶",
            PlaybackState::Paused => "⏸",
            PlaybackState::Stopped => "⏹",
        };
        let repeat = match self.repeat_mode {
            RepeatMode::None => "",
            RepeatMode::One => "🔂",
            RepeatMode::All => "🔁",
        };
        let speed_str = if (self.speed - 1.0).abs() > 0.01 {
            format!(" {}x", self.speed)
        } else {
            String::new()
        };

        format!(
            "{} {} / {} | Volume: {:.0}%{}{}",
            state,
            pos,
            dur,
            self.volume * 100.0,
            speed_str,
            repeat
        )
    }

    /// Add file to playlist
    pub fn add_to_playlist(&mut self, file: String) {
        self.playlist.push(file);
    }

    /// Remove file from playlist
    pub fn remove_from_playlist(&mut self, index: usize) {
        if index < self.playlist.len() {
            self.playlist.remove(index);
        }
    }

    /// Clear playlist
    pub fn clear_playlist(&mut self) {
        self.playlist.clear();
        self.current_index = 0;
    }

    /// Play next in playlist
    pub fn next(&mut self) -> Option<String> {
        if self.playlist.is_empty() {
            return None;
        }

        self.current_index = (self.current_index + 1) % self.playlist.len();
        Some(self.playlist[self.current_index].clone())
    }

    /// Play previous in playlist
    pub fn previous(&mut self) -> Option<String> {
        if self.playlist.is_empty() {
            return None;
        }

        self.current_index = if self.current_index == 0 {
            self.playlist.len() - 1
        } else {
            self.current_index - 1
        };
        Some(self.playlist[self.current_index].clone())
    }

    /// Get current playlist position
    pub fn playlist_position(&self) -> (usize, usize) {
        (self.current_index + 1, self.playlist.len())
    }
}

/// Playback controller for keyboard input
pub struct PlaybackController {
    /// Key mappings for playback
    pub bindings: PlaybackBindings,
}

pub struct PlaybackBindings {
    pub play_pause: String,    // Space
    pub stop: String,          // S
    pub next: String,          // N
    pub previous: String,      // P
    pub seek_forward: String,  // >
    pub seek_backward: String, // <
    pub volume_up: String,     // +
    pub volume_down: String,   // -
    pub speed_up: String,      // ]
    pub speed_down: String,    // [
    pub speed_reset: String,   // =
    pub repeat: String,        // R
    pub seek_start: String,    // Home
    pub seek_end: String,      // End
}

impl PlaybackBindings {
    /// Describe all bindings for diagnostics
    pub fn describe_all(&self) -> String {
        format!("Bindings: play={}, stop={}, seek_fwd={}, vol_up={}, speed_up={}",
            self.play_pause, self.stop, self.seek_forward, self.volume_up, self.speed_up)
    }
}

impl Default for PlaybackBindings {
    fn default() -> Self {
        Self {
            play_pause: "Space".to_string(),
            stop: "s".to_string(),
            next: "n".to_string(),
            previous: "p".to_string(),
            seek_forward: ">".to_string(),
            seek_backward: "<".to_string(),
            volume_up: "+".to_string(),
            volume_down: "-".to_string(),
            speed_up: "]".to_string(),
            speed_down: "[".to_string(),
            speed_reset: "=".to_string(),
            repeat: "r".to_string(),
            seek_start: "Home".to_string(),
            seek_end: "End".to_string(),
        }
    }
}

impl PlaybackController {
    pub fn new() -> Self {
        Self {
            bindings: PlaybackBindings::default(),
        }
    }

    /// Handle input key and return action
    pub fn handle_key(&self, key: &str) -> Option<PlaybackAction> {
        match key {
            k if k == self.bindings.play_pause => Some(PlaybackAction::TogglePlayPause),
            k if k == self.bindings.stop => Some(PlaybackAction::Stop),
            k if k == self.bindings.next => Some(PlaybackAction::Next),
            k if k == self.bindings.previous => Some(PlaybackAction::Previous),
            k if k == self.bindings.seek_forward => Some(PlaybackAction::SeekForward(Duration::from_secs(5))),
            k if k == self.bindings.seek_backward => Some(PlaybackAction::SeekBackward(Duration::from_secs(5))),
            k if k == self.bindings.volume_up => Some(PlaybackAction::VolumeUp),
            k if k == self.bindings.volume_down => Some(PlaybackAction::VolumeDown),
            k if k == self.bindings.speed_up => Some(PlaybackAction::SpeedUp),
            k if k == self.bindings.speed_down => Some(PlaybackAction::SpeedDown),
            k if k == self.bindings.speed_reset => Some(PlaybackAction::SpeedReset),
            k if k == self.bindings.repeat => Some(PlaybackAction::CycleRepeat),
            k if k == self.bindings.seek_start => Some(PlaybackAction::SkipStart),
            k if k == self.bindings.seek_end => Some(PlaybackAction::SkipEnd),
            _ => None,
        }
    }
}

/// Playback actions
#[derive(Clone, Debug)]
pub enum PlaybackAction {
    TogglePlayPause,
    Stop,
    Next,
    Previous,
    SeekForward(Duration),
    SeekBackward(Duration),
    VolumeUp,
    VolumeDown,
    SpeedUp,
    SpeedDown,
    SpeedReset,
    CycleRepeat,
    SkipStart,
    SkipEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = MediaPlayer::with_file("test.mp3".to_string(), Duration::from_secs(180));
        assert_eq!(player.current_file, "test.mp3");
        assert_eq!(player.state, PlaybackState::Stopped);
        assert_eq!(player.volume, 1.0);
        assert_eq!(player.speed, 1.0);
    }

    #[test]
    fn test_play_pause() {
        let mut player = MediaPlayer::with_file("test.mp3".to_string(), Duration::from_secs(180));
        player.play();
        assert_eq!(player.state, PlaybackState::Playing);
        player.pause();
        assert_eq!(player.state, PlaybackState::Paused);
    }

    #[test]
    fn test_seek() {
        let mut player = MediaPlayer::with_file("test.mp3".to_string(), Duration::from_secs(180));
        player.seek(Duration::from_secs(30));
        assert_eq!(player.position, Duration::from_secs(30));
    }

    #[test]
    fn test_volume() {
        let mut player = MediaPlayer::with_file("test.mp3".to_string(), Duration::from_secs(180));
        player.set_volume(0.5);
        assert_eq!(player.volume, 0.5);
        player.volume_up();
        assert!((player.volume - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_speed() {
        let mut player = MediaPlayer::with_file("test.mp3".to_string(), Duration::from_secs(180));
        player.set_speed(1.5);
        assert!((player.speed - 1.5).abs() < 0.01);
        player.speed_up();
        assert!(player.speed > 1.5);
        player.speed_normal();
        assert_eq!(player.speed, 1.0);
    }

    #[test]
    fn test_progress() {
        let mut player = MediaPlayer::with_file("test.mp3".to_string(), Duration::from_secs(200));
        player.position = Duration::from_secs(100);
        assert!((player.progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_position_string() {
        let mut player = MediaPlayer::with_file("test.mp3".to_string(), Duration::from_secs(180));
        player.position = Duration::from_secs(95);
        assert_eq!(player.position_string(), "01:35");
    }

    #[test]
    fn test_playlist() {
        let mut player = MediaPlayer::with_file("test1.mp3".to_string(), Duration::from_secs(180));
        player.add_to_playlist("test2.mp3".to_string());
        player.add_to_playlist("test3.mp3".to_string());
        assert_eq!(player.playlist.len(), 2);
        let next = player.next();
        assert_eq!(next, Some("test2.mp3".to_string()));
    }

    #[test]
    fn test_repeat_modes() {
        let mut player = MediaPlayer::with_file("test.mp3".to_string(), Duration::from_secs(180));
        assert_eq!(player.repeat_mode, RepeatMode::None);
        player.cycle_repeat();
        assert_eq!(player.repeat_mode, RepeatMode::One);
        player.cycle_repeat();
        assert_eq!(player.repeat_mode, RepeatMode::All);
        player.cycle_repeat();
        assert_eq!(player.repeat_mode, RepeatMode::None);
    }
}
