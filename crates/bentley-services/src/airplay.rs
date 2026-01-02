//! AirPlay Service - Detects AirPlay mirroring activity

use std::path::Path;
use std::fs;

/// AirPlay service for detecting mirroring activity
pub struct AirPlayService {
    state_file: String,
    cached_state: bool,
}

impl AirPlayService {
    pub fn new(state_file: String) -> Self {
        Self {
            state_file,
            cached_state: false,
        }
    }

    /// Check if AirPlay is currently active
    pub fn is_active(&self) -> bool {
        self.cached_state
    }

    /// Update state from file
    pub fn update_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.state_file);
        
        if !path.exists() {
            self.cached_state = false;
            return Ok(());
        }

        let contents = fs::read_to_string(path)?;
        let trimmed = contents.trim();
        
        self.cached_state = trimmed == "true";
        Ok(())
    }

    /// Poll state file (for use in update loop)
    pub fn poll(&mut self) {
        if let Err(e) = self.update_state() {
            tracing::warn!("Failed to update AirPlay state: {}", e);
        }
    }
}

