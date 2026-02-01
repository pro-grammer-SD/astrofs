# 🎬 Media Preview & Playback Guide

## Image Preview

### Supported Formats
- JPEG / JPG
- PNG
- GIF
- BMP
- WebP
- SVG
- TIFF
- ICO (favicons)

### Viewing Images
```
Navigate to image → Press 'v' to preview

Display shows:
✓ Image dimensions (width x height)
✓ File size
✓ Format
✓ Color depth (if available)
✓ Rendered preview in terminal (Sixel graphics)
```

### Terminal Image Rendering
```
Requirements:
- Terminal with Sixel graphics support
  - iTerm2 (macOS) - Built-in
  - Kitty (Linux/macOS) - Built-in
  - xterm with Sixel (Linux)
  
Fallback:
- ASCII art representation
- Image metadata only
```

### Example Preview Output
```
📋 File: photo.png

🖼️ IMAGE METADATA
Format: PNG
Width: 1920 px
Height: 1080 px
Size: 2048 KB

[Sixel image rendered here]
```

## 🎵 Audio Playback

### Supported Formats

| Format | Extension | Metadata | Playback |
|--------|-----------|----------|----------|
| MP3 | .mp3 | ✓ Bitrate, tags | ✓ Yes |
| FLAC | .flac | ✓ Full metadata | ✓ Yes |
| WAV | .wav | ✓ Sample rate, channels | ✓ Yes |
| AAC | .aac | ✓ Basic info | ✓ Yes |
| OGG Vorbis | .ogg | ✓ Tags | ✓ Yes |
| M4A | .m4a | ✓ iTunes tags | ✓ Yes |
| WMA | .wma | ✓ Windows media | ✓ Yes |
| Opus | .opus | ✓ Tags | ✓ Yes |

### Playing Audio

#### Start Playback
```
1. Navigate to audio file
2. Press 'v' to preview
3. Press Space to play
4. Media player UI appears at bottom of screen
```

#### Playback Controls

**Basic Control**
```
Space         Toggle play/pause
s             Stop playback
n             Next track
p             Previous track
```

**Seeking**
```
<             Rewind 5 seconds
>             Forward 5 seconds
Home          Jump to beginning
End           Jump to end
```

**Speed Control**
```
[             Decrease speed (-0.25x)
]             Increase speed (+0.25x)
=             Reset to normal speed (1.0x)

Speed range: 0.25x to 2.0x
Useful for:
- Slow down to understand content
- Speed up to save time
- Practice transcription at slower speeds
```

**Volume Control**
```
+             Volume up
-             Volume down
m             Mute/unmute

Volume levels: 0% (mute) to 100%
```

**Repeat Modes**
```
r             Cycle repeat mode

Modes:
None (🔴)     Play once and stop
One (🔂)      Repeat current track
All (🔁)      Repeat all playlist
```

### Playback Display

**Status Bar**
```
▶ 01:35 / 03:42 | Volume: 80% | 1.5x | 🔁

Shows:
- Play status icon (▶ playing, ⏸ paused, ⏹ stopped)
- Current position / Total duration
- Volume percentage
- Playback speed
- Repeat mode indicator
```

### Audio Metadata Display

**MP3 Files**
```
🎵 AUDIO METADATA
Format: MP3
Duration: 3:42
Sample Rate: 44100 Hz
Channels: Stereo (2)
Bitrate: 320 kbps
Size: 15.2 MB

ID3 Tags (if available):
Artist: The Artist
Album: Album Name
Year: 2024
```

**FLAC Files**
```
🎵 AUDIO METADATA
Format: FLAC
Duration: 3:42
Sample Rate: 48000 Hz
Channels: Stereo (2)
Bit Depth: 24-bit
Size: 35.8 MB

Tags:
Artist: The Artist
Album: Album Name
Composer: Composer Name
```

### Playlist Management

**Create Playlist**
```
1. Navigate to directory with audio files
2. Select first file (Space)
3. Select additional files (Space)
4. Press 'p' to play as playlist
5. Use n/p to navigate tracks
```

**Playlist Controls**
```
n             Next track in playlist
p             Previous track in playlist
N             Remove current from playlist
A             Add to playlist
C             Clear playlist
```

**Current Track Info**
```
Now Playing: Track 3 of 12

Displays in status bar:
[Track number] / [Total tracks]
```

## 🎬 Video Metadata

### Supported Formats
- MP4 (.mp4, .m4v)
- Matroska (.mkv)
- AVI (.avi)
- QuickTime (.mov)
- Flash Video (.flv)
- Windows Media (.wmv)
- WebM (.webm)
- 3GP (.3gp)
- MPEG-TS (.ts)

### Viewing Video Metadata

```
Navigate to video → Press 'v' to preview

Display shows:
✓ Resolution (width x height)
✓ Duration
✓ Codec information
✓ Bitrate
✓ Frame rate
✓ File size
```

### Example Video Display
```
📋 File: movie.mp4

🎬 VIDEO METADATA
Format: MP4
Width: 1920 px
Height: 1080 px
Duration: 1:45:30
Codec: H.264
Bitrate: 5000 kbps
Frame Rate: 24 fps
Size: 825 MB
```

### Video Codec Support
```
Video Codecs:
- H.264 (AVC)
- H.265 (HEVC)
- VP8, VP9
- AV1
- MPEG-4

Audio Codecs:
- AAC
- MP3
- AC3
- Opus
- Vorbis
```

## 🔄 Keyboard Reference - Media Controls

| Key | Action |
|-----|--------|
| `Space` | Play/Pause |
| `s` | Stop |
| `<` | Seek backward 5s |
| `>` | Seek forward 5s |
| `Home` | Skip to start |
| `End` | Skip to end |
| `[` | Decrease speed |
| `]` | Increase speed |
| `=` | Reset speed |
| `+` | Volume up |
| `-` | Volume down |
| `m` | Mute |
| `r` | Cycle repeat |
| `n` | Next track |
| `p` | Previous track |
| `q` | Stop & close player |

## 🎯 Advanced Media Features

### Batch Media Operations

**Process Multiple Audio Files**
```
1. Select files with Space
2. Use Ctrl+Shift+M to open Media Batch menu
3. Options:
   - Convert format
   - Change volume
   - Add to library
   - Export metadata
```

### Media Information Extraction

**Get Detailed Audio Stats**
```
1. Navigate to audio file
2. Press 'i' for detailed info
3. Shows full metadata including:
   - Bitrate
   - Sample rate
   - Duration
   - Encoding
   - ID3/Vorbis tags
```

### Quick Preview Without Playback
```
Press 'I' (capital i) for info-only mode
Shows metadata without loading audio
Perfect for large files or slow systems
```

## 🐛 Troubleshooting Media Preview

### Audio Won't Play
```
Check:
1. Terminal supports audio (some restricted environments don't)
2. File format is supported (run: astrofs --supported-formats)
3. File is not corrupted (try: ffmpeg -v error -i file.mp3)
4. Sufficient disk space for buffering
```

### Images Not Displaying
```
Check:
1. Terminal supports Sixel graphics (iTerm2, Kitty)
2. Image file is not corrupted
3. Image dimensions not too large (>10000 pixels)
4. Use 'I' to show metadata if graphics fail
```

### Video Metadata Missing
```
Common causes:
1. Container file missing duration info
2. Non-standard codec
3. Corrupted file header

Solutions:
- Remux video: ffmpeg -i input.mp4 -c copy output.mp4
- Check file: ffprobe input.mp4
- Verify format: file input.mp4
```

### Performance Issues
```
If media preview is slow:

1. Disable auto-preview
   - Settings: "auto_preview": false

2. Increase max preview size limit
   - Settings: "max_file_preview_size": 50000000

3. Check system resources
   - Run in terminal with monitoring

4. Use metadata-only mode
   - Press 'I' instead of 'v'
```

## 🎧 Audio Editing Integration

### Mark Segments for Editing
```
While playing:
1. Note timestamp at important points
2. Use Ctrl+M to mark current position
3. Marked positions saved to metadata
4. Export markers for audio editor
```

### Integration with External Players
```
Press 'E' to:
- Open in VLC
- Open in Audacity (audio)
- Open in FFmpeg processor
- Open in default player
```

## 📊 Media Statistics

### View Library Statistics
```
Press Ctrl+S to show:
- Total audio files
- Total video files
- Total image files
- Combined file size
- Average bitrate
- Most common format
```

### History & Recently Played
```
Ctrl+H shows recently played media:
- Timestamp of last play
- Playback position (resume from here)
- Play count
- Duration listened
```

---

**Related Guides:**
- [USAGE.md](USAGE.md) - Complete usage guide
- [THEMES.md](THEMES.md) - Theme customization
- [PLUGINS.md](PLUGINS.md) - Plugin development
- [Configuration](config.example.json) - Config reference
