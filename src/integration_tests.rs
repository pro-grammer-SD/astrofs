/// Integration tests that exercise all Beast Mode functionality
#[cfg(test)]
mod integration_tests {
    use std::path::PathBuf;

    #[test]
    fn test_persistence_manager_creation() {
        // Ensure PersistenceManager can be created
        let _pm = crate::persistence::PersistenceManager::default();
    }

    #[test]
    fn test_user_settings_default() {
        let settings = crate::persistence::UserSettings::default();
        assert_eq!(settings.current_theme, "default");
        assert!(settings.enabled_plugins.is_empty());
        assert_eq!(settings.parallel_search_threads, num_cpus::get());
    }

    #[test]
    fn test_theme_manager_creation() {
        let _tm = crate::theme_manager::ThemeManager::default();
    }

    #[test]
    fn test_theme_default() {
        let theme = crate::theme_manager::Theme::default();
        assert_eq!(theme.name, "default");
        assert!(!theme.colors.primary.is_empty());
        assert!(!theme.emojis.folder.is_empty());
    }

    #[test]
    fn test_plugin_api_manager() {
        let pm = crate::plugin_api::PluginManager::default();
        assert!(pm.list().is_empty());
    }

    #[test]
    fn test_media_preview_creation() {
        let mut preview = crate::media_preview::MediaPreview::new();
        assert!(preview.last_path().is_none());
        preview.clear();
        assert!(preview.last_path().is_none());
    }

    #[test]
    fn test_media_player_creation() {
        let player = crate::media_player::MediaPlayer::new();
        assert!(player.current_file.is_empty());
        assert_eq!(player.volume, 1.0);
        assert_eq!(player.speed, 1.0);
    }

    #[test]
    fn test_media_player_with_file() {
        let player = crate::media_player::MediaPlayer::with_file(
            "test.mp3".to_string(),
            std::time::Duration::from_secs(60),
        );
        assert_eq!(player.current_file, "test.mp3");
        assert_eq!(player.duration.as_secs(), 60);
    }

    #[test]
    fn test_media_player_playback_state() {
        let mut player = crate::media_player::MediaPlayer::new();
        assert_eq!(player.state, crate::media_player::PlaybackState::Stopped);
        
        player.play();
        assert_eq!(player.state, crate::media_player::PlaybackState::Playing);
        
        player.pause();
        assert_eq!(player.state, crate::media_player::PlaybackState::Paused);
        
        player.toggle();
        assert_eq!(player.state, crate::media_player::PlaybackState::Playing);
    }

    #[test]
    fn test_media_player_seek() {
        let mut player = crate::media_player::MediaPlayer::with_file(
            "test.mp3".to_string(),
            std::time::Duration::from_secs(100),
        );
        
        player.seek_forward(std::time::Duration::from_secs(10));
        assert_eq!(player.position.as_secs(), 10);
        
        player.seek_backward(std::time::Duration::from_secs(5));
        assert_eq!(player.position.as_secs(), 5);
        
        player.skip_start();
        assert_eq!(player.position.as_secs(), 0);
    }

    #[test]
    fn test_media_player_volume() {
        let mut player = crate::media_player::MediaPlayer::new();
        
        player.set_volume(0.5);
        assert_eq!(player.volume, 0.5);
        
        player.volume_up();
        assert!(player.volume > 0.5);
        
        player.volume_down();
        assert!(player.volume <= 0.5);
    }

    #[test]
    fn test_media_player_speed() {
        let mut player = crate::media_player::MediaPlayer::new();
        
        player.set_speed(1.5);
        assert_eq!(player.speed, 1.5);
        
        player.speed_up();
        assert!(player.speed > 1.5);
        
        player.speed_down();
        assert!(player.speed <= 1.5);
        
        player.speed_normal();
        assert_eq!(player.speed, 1.0);
    }

    #[test]
    fn test_media_player_repeat() {
        let mut player = crate::media_player::MediaPlayer::new();
        assert_eq!(player.repeat_mode, crate::media_player::RepeatMode::None);
        
        player.cycle_repeat();
        assert_eq!(player.repeat_mode, crate::media_player::RepeatMode::One);
        
        player.cycle_repeat();
        assert_eq!(player.repeat_mode, crate::media_player::RepeatMode::All);
        
        player.cycle_repeat();
        assert_eq!(player.repeat_mode, crate::media_player::RepeatMode::None);
    }

    #[test]
    fn test_media_player_playlist() {
        let mut player = crate::media_player::MediaPlayer::new();
        
        player.add_to_playlist("song1.mp3".to_string());
        player.add_to_playlist("song2.mp3".to_string());
        
        assert_eq!(player.playlist.len(), 2);
        assert_eq!(player.playlist_position(), (1, 2));
        
        if let Some(next) = player.next() {
            assert_eq!(next, "song2.mp3");
            assert_eq!(player.playlist_position(), (2, 2));
        }
    }

    #[test]
    fn test_playback_controller() {
        let controller = crate::media_player::PlaybackController::new();
        assert!(controller.bindings.play_pause == "Space");
        
        if let Some(action) = controller.handle_key("Space") {
            match action {
                crate::media_player::PlaybackAction::TogglePlayPause => {}
                _ => panic!("Expected TogglePlayPause action"),
            }
        } else {
            panic!("Expected Some(action)");
        }
    }

    #[test]
    fn test_media_type_detection() {
        let path = PathBuf::from("test.jpg");
        match crate::media_preview::detect_media_type(&path) {
            crate::media_preview::MediaType::Image => {}
            _ => panic!("Expected Image"),
        }
        
        let path = PathBuf::from("test.mp3");
        match crate::media_preview::detect_media_type(&path) {
            crate::media_preview::MediaType::Audio => {}
            _ => panic!("Expected Audio"),
        }
        
        let path = PathBuf::from("test.mp4");
        match crate::media_preview::detect_media_type(&path) {
            crate::media_preview::MediaType::Video => {}
            _ => panic!("Expected Video"),
        }
        
        let path = PathBuf::from("test.unknown");
        match crate::media_preview::detect_media_type(&path) {
            crate::media_preview::MediaType::Unknown => {}
            _ => panic!("Expected Unknown"),
        }
    }

    #[test]
    fn test_bookmark_state() {
        let mut settings = crate::persistence::UserSettings::default();
        let id = crate::persistence::PersistenceManager::add_bookmark(
            &mut settings,
            "test_bookmark".to_string(),
            PathBuf::from("/tmp"),
            "🔖".to_string(),
        );
        
        assert_eq!(settings.bookmarks.len(), 1);
        assert_eq!(settings.bookmarks[0].id, id);
        assert_eq!(settings.bookmarks[0].name, "test_bookmark");
    }

    #[test]
    fn test_search_history() {
        let mut settings = crate::persistence::UserSettings::default();
        crate::persistence::PersistenceManager::add_search_query(
            &mut settings,
            "test query".to_string(),
            5,
            PathBuf::from("/home"),
        );
        
        assert_eq!(settings.search_history.len(), 1);
        assert_eq!(settings.search_history[0].query, "test query");
        assert_eq!(settings.search_history[0].result_count, 5);
    }

    #[test]
    fn test_tab_state() {
        let mut settings = crate::persistence::UserSettings::default();
        let id = crate::persistence::PersistenceManager::add_tab(
            &mut settings,
            PathBuf::from("/home"),
            Some("Home".to_string()),
        );
        
        assert_eq!(settings.opened_tabs.len(), 1);
        assert_eq!(settings.opened_tabs[0].id, id);
        assert_eq!(settings.opened_tabs[0].path, PathBuf::from("/home"));
    }

    #[test]
    fn test_keybinding_set() {
        let mut settings = crate::persistence::UserSettings::default();
        crate::persistence::PersistenceManager::set_keybinding(
            &mut settings,
            "Ctrl+S".to_string(),
            "save".to_string(),
        );
        
        assert!(settings.custom_keybindings.contains_key("Ctrl+S"));
        assert_eq!(settings.custom_keybindings["Ctrl+S"], "save");
    }

    #[test]
    fn test_emoji_style() {
        let full = crate::persistence::EmojiStyle::Full;
        let none = crate::persistence::EmojiStyle::Minimal;
        let disabled = crate::persistence::EmojiStyle::Disabled;
        
        // Ensure all variants are constructible
        let _default = crate::persistence::EmojiStyle::default();
    }

    #[test]
    fn test_border_style() {
        let _rounded = crate::persistence::BorderStyle::Rounded;
        let _sharp = crate::persistence::BorderStyle::Sharp;
        let _double = crate::persistence::BorderStyle::Double;
        let _none = crate::persistence::BorderStyle::None;
        
        let _default = crate::persistence::BorderStyle::default();
    }

    #[test]
    fn test_status_bar_position() {
        let _bottom = crate::persistence::StatusBarPosition::Bottom;
        let _top = crate::persistence::StatusBarPosition::Top;
        let _hidden = crate::persistence::StatusBarPosition::Hidden;
        
        let _default = crate::persistence::StatusBarPosition::default();
    }

    #[test]
    fn test_theme_colors_parse() {
        let manager = crate::theme_manager::ThemeManager::default();
        
        if let Some((r, g, b)) = manager.parse_color("#FF0000") {
            assert_eq!((r, g, b), (255, 0, 0));
        } else {
            panic!("Failed to parse hex color");
        }
        
        if let Some((r, g, b)) = manager.parse_color("red") {
            assert_eq!((r, g, b), (255, 0, 64));
        } else {
            panic!("Failed to parse named color");
        }
    }

    #[test]
    fn test_plugin_permission() {
        let read = crate::plugin_api::PluginPermission::ReadFiles;
        let write = crate::plugin_api::PluginPermission::WriteFiles;
        let ui = crate::plugin_api::PluginPermission::RenderUI;
        
        // Ensure all permissions are distinct
        assert_ne!(read, write);
        assert_ne!(write, ui);
    }
}
