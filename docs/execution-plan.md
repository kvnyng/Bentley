# Bentley Execution Plan

This document outlines the step-by-step implementation plan for building Bentley v1 and beyond.

## Phase 0: Foundation (Weeks 1-2)

### 0.1 Project Setup ✅
- [x] Create directory structure
- [x] Set up Rust workspace
- [x] Create Cargo.toml files for all crates
- [x] Write documentation (README, architecture)

### 0.2 Core Infrastructure
- [ ] Implement `bentley-core`:
  - [ ] `View` trait definition
  - [ ] `ViewContext` struct
  - [ ] Configuration loading (`bentley.toml`, `ui.toml`)
  - [ ] Error types and result handling
- [ ] Implement `bentley-utils`:
  - [ ] Asset cache (textures, fonts)
  - [ ] Timing helpers (FPS counter, delta time)
  - [ ] Image loading utilities

### 0.3 Basic Rendering Pipeline
- [ ] Set up `bentley-app` with winit + wgpu
- [ ] Create window (fullscreen on Wayland, windowed on macOS)
- [ ] Implement basic render loop
- [ ] Add FPS counter and debug overlay
- [ ] Test on macOS (development)

## Phase 1: View System (Weeks 3-4)

### 1.1 View Manager
- [ ] Implement `ViewManager` in `bentley-app`
- [ ] View lifecycle (enter/exit)
- [ ] View transitions (fade, slide)
- [ ] Active view tracking

### 1.2 Placeholder View
- [ ] Create simple test view (solid color or pattern)
- [ ] Verify view system integration
- [ ] Test view switching

### 1.3 Configuration System
- [ ] TOML configuration parsing
- [ ] Hot-reload support (file watching)
- [ ] Configuration validation

## Phase 2: Services Layer (Weeks 5-6)

### 2.1 AirPlay Service
- [ ] Create `airplay.rs` in `bentley-services`
- [ ] State file polling (`/run/bentley/airplay_active`)
- [ ] Optional: inotify-based watching (Linux)
- [ ] Expose `airplay_active: bool` to views
- [ ] Test with mock state file

### 2.2 AirPlay Wrapper Script
- [ ] Create `system/uxplay-wrapper.sh`
- [ ] UxPlay process monitoring
- [ ] State file writing logic
- [ ] Test on SBC (or Linux VM)

### 2.3 Spotify Service (Basic)
- [ ] Create `spotify.rs` in `bentley-services`
- [ ] OAuth2 authentication flow
- [ ] Token storage and refresh
- [ ] "Currently Playing Track" API integration
- [ ] Basic polling mechanism
- [ ] Mock data mode for development

### 2.4 Service Integration
- [ ] Integrate services into `ViewContext`
- [ ] Service initialization in runtime
- [ ] Error handling and retry logic

## Phase 3: NowPlaying View (Weeks 7-8)

### 3.1 UI Components
- [ ] Text rendering (fonts)
- [ ] Image loading and texture creation (album art)
- [ ] Progress bar rendering
- [ ] Layout system (positioning, scaling)

### 3.2 NowPlaying Implementation
- [ ] Create `now_playing.rs` in `bentley-views`
- [ ] Album art display (foreground + blurred background)
- [ ] Track title and artist text
- [ ] Playback progress bar
- [ ] Update loop with Spotify service integration

### 3.3 Styling and Polish
- [ ] Load styling from `ui.toml`
- [ ] Smooth animations (fade-in, transitions)
- [ ] Responsive layout for different screen sizes

## Phase 4: AirPlay Integration (Weeks 9-10)

### 4.1 Background Degradation
- [ ] FPS throttling based on AirPlay state
- [ ] Animation pausing/simplification
- [ ] Service polling throttling
- [ ] State preservation during degradation

### 4.2 UxPlay Integration
- [ ] Install and configure UxPlay on SBC
- [ ] Create `system/uxplay.service`
- [ ] Test wrapper script with real AirPlay connections
- [ ] Verify state file updates

### 4.3 End-to-End Testing
- [ ] Test AirPlay connection/disconnection flow
- [ ] Verify Bentley resumes correctly
- [ ] Performance testing (CPU, memory, GPU)

## Phase 5: SBC Deployment (Weeks 11-12)

### 5.1 Armbian Setup
- [ ] Install Armbian 25 CLI/server image
- [ ] Install dependencies (Weston, UxPlay, GStreamer)
- [ ] Configure DRM/KMS
- [ ] Test HDMI output

### 5.2 Weston Configuration
- [ ] Create `system/weston.ini`
- [ ] Configure kiosk-shell
- [ ] Set up autostart
- [ ] Test fullscreen behavior

### 5.3 Systemd Services
- [ ] Create `system/bentley.service`
- [ ] Create `system/uxplay.service`
- [ ] Install service files
- [ ] Configure service dependencies
- [ ] Test auto-start on boot

### 5.4 Build and Deploy
- [ ] Cross-compilation setup (or native build on SBC)
- [ ] Install binary to `/usr/local/bin/bentley`
- [ ] Copy configuration files to `/etc/bentley/`
- [ ] Create runtime directories (`/run/bentley/`, `/var/lib/bentley/`, etc.)
- [ ] Test full deployment

### 5.5 Optimization
- [ ] Profile on SBC hardware
- [ ] Optimize rendering (reduce draw calls)
- [ ] Memory usage optimization
- [ ] CPU usage optimization

## Phase 6: Polish and Documentation (Week 13)

### 6.1 Error Handling
- [ ] Comprehensive error messages
- [ ] Graceful degradation on service failures
- [ ] Logging and diagnostics

### 6.2 Documentation
- [ ] User guide
- [ ] Deployment guide
- [ ] Troubleshooting guide
- [ ] Developer guide

### 6.3 Testing
- [ ] Unit tests for core components
- [ ] Integration tests for services
- [ ] End-to-end testing on SBC

## Phase 7: Future Views (Post-v1)

### 7.1 Wall Art View
- [ ] Shader-based generative art
- [ ] Time-based animations
- [ ] Multiple art styles

### 7.2 Audio Visualizer
- [ ] Audio capture service (ALSA/PulseAudio)
- [ ] FFT pipeline
- [ ] Real-time frequency visualization
- [ ] Multiple visualization modes

### 7.3 Lyric Follower
- [ ] Lyrics API integration
- [ ] Synchronized lyric display
- [ ] Styling and animations

### 7.4 RL Snake
- [ ] Snake game implementation
- [ ] RL model integration (TensorFlow Lite)
- [ ] Autonomous gameplay
- [ ] Visual polish

### 7.5 Autonomous Tetris
- [ ] Tetris game implementation
- [ ] RL model integration
- [ ] Autonomous gameplay
- [ ] Visual polish

## Implementation Priorities

### Must-Have (v1)
1. ✅ Project setup and documentation
2. Core view system
3. AirPlay service and integration
4. NowPlaying view
5. SBC deployment

### Nice-to-Have (v1)
- Hot-reload for configuration
- Debug overlay improvements
- Performance optimizations

### Future (Post-v1)
- Additional views (Wall Art, Visualizer, etc.)
- View scheduling and prioritization
- Audio processing
- Machine learning integration

## Development Workflow

### Daily Development
1. Work on macOS for rapid iteration
2. Test rendering in windowed mode
3. Use mock data for services
4. Regular commits with descriptive messages

### Weekly Testing
1. Test on Linux VM (Wayland)
2. Verify service integrations
3. Performance profiling
4. Documentation updates

### Pre-Deployment
1. Full test on SBC hardware
2. End-to-end integration testing
3. Performance validation
4. Documentation review

## Risk Mitigation

### Technical Risks
- **wgpu compatibility on SBC**: Test early, have OpenGL fallback
- **Weston stability**: Monitor logs, implement restart logic
- **UxPlay reliability**: Test thoroughly, handle edge cases
- **Performance on low-power hardware**: Profile early, optimize continuously

### Process Risks
- **Scope creep**: Stick to v1 goals, defer features to post-v1
- **Timeline delays**: Prioritize core features, cut nice-to-haves if needed
- **Hardware availability**: Test on VM first, order SBC early

## Success Criteria (v1)

- [ ] Boots directly into fullscreen Bentley on SBC
- [ ] NowPlaying view displays Spotify track information
- [ ] AirPlay mirroring works seamlessly
- [ ] Bentley resumes automatically after AirPlay
- [ ] Runs reliably for extended periods (24+ hours)
- [ ] Low resource usage (<50% CPU, <500MB RAM)

## Next Steps

1. **Immediate**: Begin Phase 0.2 (Core Infrastructure)
2. **This Week**: Set up basic rendering pipeline
3. **This Month**: Complete view system and services layer
4. **Next Month**: Implement NowPlaying view and AirPlay integration

---

*This plan is a living document and will be updated as the project evolves.*

