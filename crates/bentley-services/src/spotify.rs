//! Spotify Service - Integration with Spotify Web API

use serde::{Deserialize, Serialize};

/// Currently playing track information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyTrack {
    pub title: String,
    pub artist: Vec<String>,
    pub album: String,
    pub album_art_url: String,
    pub progress_ms: u32,
    pub duration_ms: u32,
    pub is_playing: bool,
}

/// Spotify service for fetching playback state
pub struct SpotifyService {
    // TODO: Add service state (client, tokens, etc.)
}

impl SpotifyService {
    pub fn new() -> Self {
        Self {
            // TODO: Initialize
        }
    }

    /// Get the currently playing track
    pub async fn get_current_track(&self) -> Result<Option<SpotifyTrack>, Box<dyn std::error::Error>> {
        // TODO: Implement Spotify Web API integration
        Ok(None)
    }

    /// Check if Spotify is currently playing
    pub async fn is_playing(&self) -> bool {
        // TODO: Implement
        false
    }
}

