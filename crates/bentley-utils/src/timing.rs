//! Timing utilities - FPS counter, delta time, etc.

use std::time::{Duration, Instant};

/// FPS counter
pub struct FpsCounter {
    frame_count: u32,
    last_update: Instant,
    current_fps: f32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_update: Instant::now(),
            current_fps: 0.0,
        }
    }

    /// Update the FPS counter (call once per frame)
    pub fn update(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);

        if elapsed >= Duration::from_secs(1) {
            self.current_fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_update = now;
        }
    }

    /// Get the current FPS
    pub fn fps(&self) -> f32 {
        self.current_fps
    }
}

/// Delta time calculator
pub struct DeltaTime {
    last_frame: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
        }
    }

    /// Calculate delta time since last call
    pub fn update(&mut self) -> f32 {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        dt
    }
}

