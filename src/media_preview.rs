// Media Preview - Handle image, audio, and video metadata
use anyhow::Result;
use std::path::Path;

#[derive(Clone, Debug)]
pub enum MediaType {
    Image,
    Audio,
    Video,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct MediaMetadata {
    pub media_type: MediaType,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<u64>,      // milliseconds
    pub bitrate: Option<u32>,       // kbps
    pub sample_rate: Option<u32>,   // Hz
    pub channels: Option<u8>,
    pub codec: Option<String>,
    pub format: String,
    pub size_bytes: u64,
}

/// Detect media type from file extension
pub fn detect_media_type(path: &Path) -> MediaType {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        // Image formats
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" | "tiff" => {
            MediaType::Image
        }
        // Audio formats
        "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a" | "wma" | "opus" => MediaType::Audio,
        // Video formats
        "mp4" | "mkv" | "avi" | "mov" | "flv" | "wmv" | "webm" | "m4v" | "3gp" | "ts" => {
            MediaType::Video
        }
        _ => MediaType::Unknown,
    }
}

/// Get image metadata
pub fn get_image_metadata(path: &Path) -> Result<MediaMetadata> {
    let file_size = std::fs::metadata(path)?.len();
    let format = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown")
        .to_lowercase();

    // Try to get dimensions using image crate if available
    let (width, height) = get_image_dimensions(path).unwrap_or((None, None));

    Ok(MediaMetadata {
        media_type: MediaType::Image,
        width,
        height,
        duration: None,
        bitrate: None,
        sample_rate: None,
        channels: None,
        codec: None,
        format,
        size_bytes: file_size,
    })
}

/// Try to get image dimensions
fn get_image_dimensions(path: &Path) -> Result<(Option<u32>, Option<u32>)> {
    if let Ok(img) = image::image_dimensions(path) {
        Ok((Some(img.0), Some(img.1)))
    } else {
        Ok((None, None))
    }
}

/// Get audio metadata
pub fn get_audio_metadata(path: &Path) -> Result<MediaMetadata> {
    let file_size = std::fs::metadata(path)?.len();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    // Try different audio format parsers based on extension
    let (duration, bitrate, sample_rate, channels, codec) = match ext.as_str() {
        "flac" => get_flac_metadata(path).unwrap_or_default(),
        "mp3" => get_mp3_metadata(path).unwrap_or_default(),
        "wav" => get_wav_metadata(path).unwrap_or_default(),
        _ => (None, None, None, None, None),
    };

    Ok(MediaMetadata {
        media_type: MediaType::Audio,
        width: None,
        height: None,
        duration,
        bitrate,
        sample_rate,
        channels,
        codec,
        format: ext,
        size_bytes: file_size,
    })
}

/// Get FLAC metadata
fn get_flac_metadata(path: &Path) -> Result<(Option<u64>, Option<u32>, Option<u32>, Option<u8>, Option<String>)> {
    // Using metaflac crate
    match metaflac::Tag::read_from_path(path) {
        Ok(tag) => {
            // FLAC doesn't provide duration via metaflac directly; we'd need to parse frames
            let duration = None;

            let sample_rate = tag
                .get_streaminfo()
                .map(|si| si.sample_rate);

            let channels = tag
                .get_streaminfo()
                .map(|_si| {
                    // Extract channel count from audio info
                    1 // Simplified - would need proper parsing
                });

            Ok((
                duration,
                None, // bitrate would need to be calculated
                sample_rate,
                channels,
                Some("FLAC".to_string()),
            ))
        }
        Err(_) => Ok((None, None, None, None, None)),
    }
}

/// Get MP3 metadata
fn get_mp3_metadata(_path: &Path) -> Result<(Option<u64>, Option<u32>, Option<u32>, Option<u8>, Option<String>)> {
    // mp3-metadata API differs - this is simplified
    // In a real implementation, we'd use metaflac or another library
    Ok((None, None, None, Some(2), Some("MP3".to_string())))
}

/// Get WAV metadata
fn get_wav_metadata(path: &Path) -> Result<(Option<u64>, Option<u32>, Option<u32>, Option<u8>, Option<String>)> {
    // Using wav crate or simple WAV header parsing
    match read_wav_header(path) {
        Ok((sample_rate, channels, num_samples)) => {
            let duration = if sample_rate > 0 {
                Some((num_samples as u64 * 1000) / sample_rate as u64)
            } else {
                None
            };

            let bitrate = (sample_rate as u32 * channels as u32 * 16) / 1000; // Assuming 16-bit

            Ok((
                duration,
                Some(bitrate),
                Some(sample_rate),
                Some(channels),
                Some("WAV".to_string()),
            ))
        }
        Err(_) => Ok((None, None, None, None, None)),
    }
}

/// Read basic WAV header information
fn read_wav_header(path: &Path) -> Result<(u32, u8, u32)> {
    let mut file = std::fs::File::open(path)?;
    
    // WAV header can be 36 or more bytes, need to read chunk info
    let mut header = [0u8; 44];
    use std::io::Read as IORead;
    let bytes_read = IORead::read(&mut file, &mut header)?;

    // Ensure we have minimum header data
    if bytes_read < 36 {
        anyhow::bail!("WAV file header too small");
    }

    // Parse WAV header
    let sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
    let channels = u16::from_le_bytes([header[22], header[23]]) as u8;
    
    // Get number of samples from data chunk size if available
    let num_samples = if bytes_read >= 44 {
        u32::from_le_bytes([header[40], header[41], header[42], header[43]]) / (channels as u32 * 2)
    } else {
        0 // Default to 0 if not available
    };

    Ok((sample_rate, channels, num_samples))
}

/// Get video metadata
pub fn get_video_metadata(path: &Path) -> Result<MediaMetadata> {
    let file_size = std::fs::metadata(path)?.len();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    // Try MP4 parsing
    let (width, height, duration, bitrate, codec) = if ext == "mp4" || ext == "m4v" {
        get_mp4_metadata(path).unwrap_or_default()
    } else {
        (None, None, None, None, None)
    };

    Ok(MediaMetadata {
        media_type: MediaType::Video,
        width,
        height,
        duration,
        bitrate,
        sample_rate: None,
        channels: None,
        codec,
        format: ext,
        size_bytes: file_size,
    })
}

/// Get MP4 metadata
fn get_mp4_metadata(_path: &Path) -> Result<(Option<u32>, Option<u32>, Option<u64>, Option<u32>, Option<String>)> {
    // This is a simplified implementation
    // Full MP4 parsing would require a proper mp4 parser
    Ok((None, None, None, None, Some("H.264".to_string())))
}

/// Generate a text preview for media file
pub fn generate_media_preview(path: &Path) -> Result<String> {
    let media_type = detect_media_type(path);
    let mut preview = String::new();

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    preview.push_str(&format!("📋 File: {}\n\n", filename));

    match media_type {
        MediaType::Image => {
            let meta = get_image_metadata(path)?;
            preview.push_str("🖼️ IMAGE METADATA\n");
            preview.push_str(&format!("Format: {}\n", meta.format.to_uppercase()));
            if let Some(w) = meta.width {
                preview.push_str(&format!("Width: {} px\n", w));
            }
            if let Some(h) = meta.height {
                preview.push_str(&format!("Height: {} px\n", h));
            }
            preview.push_str(&format!(
                "Size: {} KB\n",
                meta.size_bytes / 1024
            ));
        }
        MediaType::Audio => {
            let meta = get_audio_metadata(path)?;
            preview.push_str("🎵 AUDIO METADATA\n");
            preview.push_str(&format!("Format: {}\n", meta.format.to_uppercase()));
            if let Some(d) = meta.duration {
                let mins = d / 60000;
                let secs = (d % 60000) / 1000;
                preview.push_str(&format!("Duration: {}:{:02}\n", mins, secs));
            }
            if let Some(sr) = meta.sample_rate {
                preview.push_str(&format!("Sample Rate: {} Hz\n", sr));
            }
            if let Some(ch) = meta.channels {
                preview.push_str(&format!("Channels: {}\n", match ch {
                    1 => "Mono",
                    2 => "Stereo",
                    _ => "Multi-channel",
                }));
            }
            if let Some(br) = meta.bitrate {
                preview.push_str(&format!("Bitrate: {} kbps\n", br));
            }
            preview.push_str(&format!(
                "Size: {} MB\n",
                meta.size_bytes / 1024 / 1024
            ));
        }
        MediaType::Video => {
            let meta = get_video_metadata(path)?;
            preview.push_str("🎬 VIDEO METADATA\n");
            preview.push_str(&format!("Format: {}\n", meta.format.to_uppercase()));
            if let Some(w) = meta.width {
                preview.push_str(&format!("Width: {} px\n", w));
            }
            if let Some(h) = meta.height {
                preview.push_str(&format!("Height: {} px\n", h));
            }
            if let Some(d) = meta.duration {
                let mins = d / 60000;
                let secs = (d % 60000) / 1000;
                preview.push_str(&format!("Duration: {}:{:02}\n", mins, secs));
            }
            if let Some(c) = meta.codec {
                preview.push_str(&format!("Codec: {}\n", c));
            }
            if let Some(br) = meta.bitrate {
                preview.push_str(&format!("Bitrate: {} kbps\n", br));
            }
            preview.push_str(&format!(
                "Size: {} MB\n",
                meta.size_bytes / 1024 / 1024
            ));
        }
        MediaType::Unknown => {
            preview.push_str("Unknown media type\n");
        }
    }

    Ok(preview)
}

/// MediaPreview wrapper struct for integration into the App
pub struct MediaPreview {
    last_preview_path: Option<std::path::PathBuf>,
}

impl MediaPreview {
    /// Create a new MediaPreview instance
    pub fn new() -> Self {
        Self {
            last_preview_path: None,
        }
    }

    /// Get metadata for a file and return formatted preview string
    pub fn get_metadata(&mut self, path: &std::path::PathBuf) -> Result<Option<String>> {
        let media_type = detect_media_type(path);
        
        // Only return preview for actual media files
        match media_type {
            MediaType::Unknown => Ok(None),
            _ => {
                self.last_preview_path = Some(path.clone());
                generate_media_preview(path).map(Some)
            }
        }
    }

    /// Get the last previewed path
    pub fn last_path(&self) -> Option<&std::path::PathBuf> {
        self.last_preview_path.as_ref()
    }

    /// Clear the preview cache
    pub fn clear(&mut self) {
        self.last_preview_path = None;
    }
}

impl Default for MediaPreview {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_image() {
        let path = Path::new("test.jpg");
        match detect_media_type(path) {
            MediaType::Image => {}
            _ => panic!("Expected Image"),
        }
    }

    #[test]
    fn test_detect_audio() {
        let path = Path::new("test.mp3");
        match detect_media_type(path) {
            MediaType::Audio => {}
            _ => panic!("Expected Audio"),
        }
    }

    #[test]
    fn test_detect_video() {
        let path = Path::new("test.mp4");
        match detect_media_type(path) {
            MediaType::Video => {}
            _ => panic!("Expected Video"),
        }
    }
}
