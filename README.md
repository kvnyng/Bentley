# Bentley

**Bentley** is a modular, fullscreen visual runtime designed for small single-board computers (SBCs) connected to HDMI displays. It functions as ambient, intelligent smart home decor—transforming an SBC and display into a programmable visual object that blends software, art, and ambient computation into the home environment.

## Overview

Rather than being a single-purpose application, Bentley is a **view-based visual platform**. It hosts multiple independent visual "views"—music-aware displays, generative art, simulations, and autonomous games—within a single, appliance-like runtime.

### Key Characteristics

- **Appliance-like behavior**: Boots directly into a fullscreen experience with no desktop environment
- **Context-aware**: Passively reacts to music, time, and activity
- **AirPlay integration**: Seamlessly yields to AirPlay mirroring and resumes automatically
- **Modular architecture**: View-based system supporting multiple visual modes
- **Cross-platform development**: Single codebase supporting both SBC deployment and macOS development

## Target Platform

- **Hardware**: Orange Pi 2W (and similar SBCs)
- **OS**: Armbian 25 (CLI/server image recommended)
- **Display**: HDMI via DRM/KMS
- **Display Server**: Weston with kiosk-shell
- **Runtime**: Rust + wgpu
- **AirPlay**: UxPlay (rootless user service)

## Architecture

Bentley consists of several key components:

1. **Weston (kiosk-shell)**: Runs on DRM/KMS, forces fullscreen kiosk behavior
2. **Bentley Runtime**: Single binary hosting all views, manages rendering and lifecycle
3. **UxPlay**: Runs continuously as a user service for AirPlay mirroring
4. **Systemd**: Manages services, restarts, and logging

For detailed architecture information, see [docs/architecture.md](docs/architecture.md).

## Project Structure

```
bentley/
├── crates/
│   ├── bentley-app/            # Main runtime binary
│   ├── bentley-core/           # Shared primitives
│   ├── bentley-views/          # All views
│   ├── bentley-services/       # Spotify, AirPlay, audio, ML
│   └── bentley-utils/          # Shared utilities
├── assets/
│   ├── fonts/
│   ├── shaders/
│   └── images/
├── config/
│   ├── bentley.toml
│   └── ui.toml
├── system/
│   ├── weston.ini
│   ├── bentley.service
│   ├── uxplay.service
│   └── uxplay-wrapper.sh
├── scripts/
├── docs/
└── README.md
```

## Views

Bentley supports multiple visual views:

- **NowPlaying**: Displays currently playing Spotify track with album art and progress
- **LyricFollower**: Synchronized lyrics display (future)
- **AudioVisualizer**: Real-time audio visualization (future)
- **Wall Art**: Generative art shaders (future)
- **RL Snake**: Autonomous Snake game with reinforcement learning (future)
- **Tetris**: Autonomous Tetris (future)

## Development

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- For macOS development: Xcode Command Line Tools
- For SBC deployment: Armbian 25, Weston, UxPlay

### Building

```bash
# Build all crates
cargo build

# Build release binary
cargo build --release

# Run on macOS (development mode)
cargo run --bin bentley
```

### Development Workflow

- **Hot-reload**: UI layout (`ui.toml`), shaders (`.wgsl`), mock data modes
- **Debug overlay**: FPS, active view, AirPlay state (press F1)
- **Cross-platform**: Uses `winit + wgpu` for macOS (Metal) and Wayland (Vulkan/OpenGL)

## Deployment

### SBC Setup

1. Install Armbian 25 CLI/server image
2. Install dependencies:
   ```bash
   sudo apt-get update
   sudo apt-get install -y weston kiosk-shell gstreamer1.0-tools \
     gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
     gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
     avahi-daemon libavahi-client-dev
   ```
3. Build and install Bentley:
   ```bash
   cargo build --release
   sudo cp target/release/bentley /usr/local/bin/
   ```
4. Install systemd services (see `system/` directory)
5. Enable and start services:
   ```bash
   systemctl --user enable bentley.service
   systemctl --user enable uxplay.service
   systemctl --user start bentley.service
   systemctl --user start uxplay.service
   ```

### Configuration

Edit configuration files in `/etc/bentley/`:
- `bentley.toml`: Runtime configuration
- `ui.toml`: UI layout and styling

## AirPlay Integration

Bentley seamlessly integrates with AirPlay via UxPlay:

- When AirPlay connects, Bentley automatically reduces FPS and pauses animations
- AirPlay content temporarily replaces Bentley's visuals
- When mirroring ends, Bentley resumes automatically
- State is preserved during interruption

AirPlay activity is detected via a wrapper script that monitors UxPlay and writes state to `/run/bentley/airplay_active`.

## Goals

### Primary Goals (v1)

- ✅ Boot directly into fullscreen runtime
- ✅ Run reliably on low-power SBC using HDMI output
- ✅ NowPlaying view with Spotify integration
- ✅ Seamless AirPlay mirroring via UxPlay
- ✅ Background degradation during AirPlay

### Long-Term Goals

- Multiple view support with scheduling and prioritization
- Context-driven view switching
- Audio capture + FFT pipelines
- RL inference engines
- Generative art shaders
- Night modes, dimming, and idle states

## Implementation Plan

For a detailed roadmap and implementation steps, see [docs/execution-plan.md](docs/execution-plan.md).

## License

MIT OR Apache-2.0

## Contributing

This project is in early development. Contributions and feedback are welcome!
