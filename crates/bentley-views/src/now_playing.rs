//! NowPlaying View - Displays currently playing Spotify track

use bentley_core::view::View;
use bentley_core::context::ViewContext;
use wgpu::CommandEncoder;
use wgpu::TextureView;

pub struct NowPlayingView {
    // TODO: Add view state
}

impl NowPlayingView {
    pub fn new() -> Self {
        Self {
            // TODO: Initialize
        }
    }

    /// Render the view
    /// 
    /// # Arguments
    /// * `encoder` - wgpu command encoder
    /// * `view` - Target texture view to render to
    pub fn render(
        &mut self,
        _encoder: &mut CommandEncoder,
        _view: &TextureView,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Render album art, track info, progress bar
        Ok(())
    }
}

impl View for NowPlayingView {
    fn on_enter(&mut self, _context: &mut ViewContext) {
        // TODO: Initialize view state
    }

    fn on_exit(&mut self) {
        // TODO: Cleanup
    }

    fn update(&mut self, _dt: f32, _context: &ViewContext) {
        // TODO: Update view state
    }
}

