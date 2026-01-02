//! Asset Cache - Manages textures, fonts, and shaders

use std::collections::HashMap;
use std::sync::Arc;
use wgpu::Texture;

/// Asset cache for managing loaded resources
pub struct AssetCache {
    textures: HashMap<String, Arc<Texture>>,
    // TODO: Add fonts, shaders, etc.
}

impl AssetCache {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Load a texture from a file path
    pub fn load_texture(
        &mut self,
        _path: &str,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) -> Result<Arc<Texture>, Box<dyn std::error::Error>> {
        // TODO: Implement texture loading
        Err("Not implemented".into())
    }

    /// Get a cached texture
    pub fn get_texture(&self, key: &str) -> Option<Arc<Texture>> {
        self.textures.get(key).cloned()
    }
}

