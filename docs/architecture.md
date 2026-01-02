# Bentley Architecture

This document describes the system architecture, component interactions, and design decisions for Bentley.

## High-Level Architecture

Bentley is built as a modular, view-based visual runtime that operates as an appliance-like system on single-board computers. The architecture is designed to be:

- **Modular**: Views are independent, self-contained visual modes
- **Service-oriented**: External integrations (Spotify, AirPlay) are abstracted as services
- **Cross-platform**: Single codebase supports both macOS development and SBC deployment
- **Efficient**: Optimized for low-power hardware with graceful degradation

## System Components

### 1. Display Server: Weston (kiosk-shell)

**Purpose**: Provides the Wayland compositor and enforces fullscreen kiosk behavior.

**Configuration**: `system/weston.ini`
- Runs on DRM/KMS (direct rendering manager/kernel mode setting)
- Forces fullscreen mode with no window decorations
- Controls which application occupies the display
- Handles input (though Bentley is primarily passive)

**Key Features**:
- Kiosk-shell prevents window management UI
- Automatic fullscreen for all clients
- DRM backend for direct hardware access

### 2. Bentley Runtime

**Purpose**: Core application hosting all views and managing rendering lifecycle.

**Architecture**:
```
┌─────────────────────────────────────┐
│      Bentley Runtime (bentley-app)  │
├─────────────────────────────────────┤
│  View Manager                         │
│  ├── Active View                      │
│  ├── View Transitions                 │
│  └── View Lifecycle                   │
├─────────────────────────────────────┤
│  Rendering Engine (wgpu)             │
│  ├── Frame Pacing                    │
│  ├── Asset Caching                   │
│  └── Render Pipeline                 │
├─────────────────────────────────────┤
│  ViewContext                         │
│  ├── Services (Spotify, AirPlay)     │
│  ├── Asset Cache                     │
│  ├── Timing Helpers                  │
│  └── Configuration                   │
└─────────────────────────────────────┘
```

**Key Responsibilities**:
- View lifecycle management (enter/exit, transitions)
- Frame pacing and rendering coordination
- Asset caching (textures, fonts, shaders)
- Service integration and state management
- Configuration loading and hot-reload

### 3. View System

**Design Pattern**: Each view implements a shared trait/interface:

```rust
trait View {
    fn on_enter(&mut self, context: &mut ViewContext);
    fn on_exit(&mut self);
    fn update(&mut self, dt: f32, context: &ViewContext);
    fn render(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
}
```

**View Types**:
- **NowPlayingView**: Spotify track display with album art
- **WallArtView**: Generative art shaders
- **VisualizerView**: Real-time audio visualization
- **LyricFollowerView**: Synchronized lyrics
- **SnakeView**: Autonomous RL-based Snake game
- **TetrisView**: Autonomous Tetris game

**View Manager**:
- Tracks currently active view
- Handles transitions (fade, slide, etc.)
- Manages view priority and scheduling (future)
- Enables idle-state switching (future)

### 4. Services Layer

**Purpose**: Abstract external integrations and provide unified API to views.

#### Spotify Service

**Implementation**: `crates/bentley-services/src/spotify.rs`

**API**:
- Spotify Web API integration
- OAuth2 authentication flow
- Polling for "Currently Playing Track"
- Album art fetching and caching

**Data Model**:
```rust
pub struct SpotifyTrack {
    pub title: String,
    pub artist: Vec<String>,
    pub album: String,
    pub album_art_url: String,
    pub progress_ms: u32,
    pub duration_ms: u32,
    pub is_playing: bool,
}
```

**Performance**:
- Polling interval: 1-2 seconds (configurable)
- Throttled during AirPlay activity
- Album art cached locally

#### AirPlay Service

**Implementation**: `crates/bentley-services/src/airplay.rs`

**Detection Strategy**:
Since Wayland clients cannot enumerate other clients' windows, AirPlay activity is detected via:

1. **Wrapper Script**: `system/uxplay-wrapper.sh`
   - Launches UxPlay
   - Monitors UxPlay lifecycle and output
   - Infers AirPlay session start/stop
   - Writes state to `/run/bentley/airplay_active`

2. **State File**: `/run/bentley/airplay_active`
   - Contains single boolean value: `true` or `false`
   - Updated by wrapper script
   - Polled by Bentley runtime (or watched via inotify)

**API**:
```rust
pub struct AirPlayService {
    pub airplay_active: bool,
}

impl AirPlayService {
    pub fn is_active(&self) -> bool;
    pub fn update_state(&mut self);
}
```

**Integration**:
- Bentley polls state file periodically (or uses inotify)
- Views adapt behavior based on `airplay_active` flag
- Runtime reduces FPS and pauses animations when active

#### Audio Service (Future)

**Purpose**: Audio capture and FFT processing for visualizers.

**Planned Features**:
- ALSA/PulseAudio capture
- FFT pipeline for frequency analysis
- Real-time audio buffer processing

### 5. UxPlay Integration

**Purpose**: Provides AirPlay mirroring capability.

**Configuration**: `system/uxplay.service`

**Operation**:
- Runs continuously as a user systemd service
- Creates fullscreen window during AirPlay mirroring
- Automatically relinquishes control when mirroring ends
- Wrapped by `uxplay-wrapper.sh` for state detection

**Wrapper Script Behavior**:
```bash
# Pseudocode
1. Start UxPlay
2. Monitor process and output
3. Detect connection → write "true" to /run/bentley/airplay_active
4. Detect disconnection → write "false" to /run/bentley/airplay_active
5. Exit when UxPlay exits
```

## Display and Interruption Model

### Normal Operation

1. Weston launches in kiosk mode
2. Bentley occupies HDMI output fullscreen
3. Active view renders at full FPS (typically 60 FPS)
4. Services poll at normal intervals

### AirPlay Interruption

**Sequence**:
1. AirPlay client connects
2. UxPlay wrapper detects connection → writes `true` to state file
3. Bentley polls/watches state file → detects AirPlay active
4. Bentley enters "nicer" mode:
   - Reduces FPS (60 → 10)
   - Pauses or simplifies animations
   - Slows external polling (Spotify, etc.)
   - Preserves internal state
5. UxPlay creates fullscreen window
6. Weston displays UxPlay window (Bentley still renders in background)
7. When mirroring ends:
   - UxPlay exits/closes window
   - Wrapper writes `false` to state file
   - Bentley detects change → resumes full FPS
   - Bentley becomes visible again (no restart needed)

### Background Degradation ("Nicer" Mode)

While AirPlay is active, Bentley:
- **Rendering**: 60 FPS → 10 FPS
- **Animations**: Paused or simplified
- **Network Polling**: Spotify polling interval increased (e.g., 1s → 5s)
- **State Preservation**: All view state maintained for instant recovery

## Rendering Architecture

### Graphics Backend: wgpu

**Why wgpu?**
- Cross-platform: Metal (macOS), Vulkan (Linux), OpenGL (fallback)
- Modern graphics API abstraction
- Efficient resource management
- Shader support (WGSL)

### Rendering Pipeline

```
┌─────────────┐
│   View      │
│  (update)   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  View       │
│  (render)   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  wgpu       │
│  Encoder    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  GPU        │
│  (DRM/KMS)  │
└─────────────┘
```

### Asset Management

**Asset Cache**: `crates/bentley-utils/src/assets.rs`

**Cached Resources**:
- Textures (album art, images)
- Fonts (TTF/OTF)
- Shaders (WGSL)
- Compiled pipelines

**Loading Strategy**:
- Lazy loading on first use
- LRU eviction for memory management
- Background preloading for known assets

## Configuration System

### Runtime Configuration: `config/bentley.toml`

```toml
[display]
fps = 60
fullscreen = true

[airplay]
poll_interval_ms = 100
state_file = "/run/bentley/airplay_active"

[spotify]
poll_interval_ms = 2000
client_id = "..."
client_secret = "..."
redirect_uri = "http://localhost:8888/callback"

[views]
default = "now_playing"
```

### UI Configuration: `config/ui.toml`

```toml
[now_playing]
album_art_size = 512
font_size_title = 48
font_size_artist = 32
progress_bar_height = 4
```

**Hot-Reload**: Configuration files can be watched and reloaded without restart.

## Development vs. Deployment

### macOS Development

- **Backend**: Metal via wgpu
- **Window**: Windowed mode (for debugging)
- **Input**: Keyboard/mouse for debug controls
- **Services**: Mock data modes available

### SBC Deployment

- **Backend**: Vulkan/OpenGL via wgpu
- **Window**: Fullscreen via Wayland
- **Input**: Minimal (passive operation)
- **Services**: Real integrations (Spotify, AirPlay)

## Systemd Integration

### Service Files

**`system/bentley.service`**:
```ini
[Unit]
Description=Bentley Visual Runtime
After=weston.service
Requires=weston.service

[Service]
Type=simple
ExecStart=/usr/local/bin/bentley
Restart=always
RestartSec=5
Environment=WAYLAND_DISPLAY=wayland-0

[Install]
WantedBy=default.target
```

**`system/uxplay.service`**:
```ini
[Unit]
Description=UxPlay AirPlay Server
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/uxplay-wrapper.sh
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
```

### Service Dependencies

```
weston.service
    └── bentley.service
    └── uxplay.service
```

## Future Extensions

### View Scheduling

- Time-based view switching (e.g., Wall Art at night)
- Priority system (e.g., NowPlaying overrides during music)
- Context-driven transitions

### Audio Processing

- ALSA/PulseAudio capture
- FFT pipeline for frequency analysis
- Real-time visualization data

### Machine Learning

- RL inference engines for Snake/Tetris
- On-device model inference
- TensorFlow Lite or similar

### Generative Art

- Shader-based procedural generation
- Time-based animations
- Interactive elements (future)

## Performance Considerations

### Memory Management

- Asset cache with LRU eviction
- Texture compression where possible
- Efficient buffer management in wgpu

### CPU Optimization

- Frame pacing to avoid unnecessary work
- Background degradation during AirPlay
- Efficient polling intervals

### GPU Optimization

- Batch rendering where possible
- Shader optimization
- Texture atlasing for small images

## Security Considerations

- Spotify OAuth2 tokens stored securely
- No network-exposed services (except AirPlay via UxPlay)
- Minimal attack surface (no desktop environment)
- User service isolation (systemd)

