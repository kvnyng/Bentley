//! ViewContext - Shared context provided to all views

use crate::config::Config;

/// Context shared across all views
/// Provides access to services, assets, and configuration
pub struct ViewContext {
    /// Runtime configuration
    pub config: Config,
    
    // Services will be added here
    // pub spotify: Arc<SpotifyService>,
    // pub airplay: Arc<AirPlayService>,
    
    // Asset cache will be added here
    // pub assets: Arc<AssetCache>,
}

impl ViewContext {
    /// Create a new ViewContext
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }
}

