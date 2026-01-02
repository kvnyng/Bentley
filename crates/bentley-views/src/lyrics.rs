//! Lyric Follower View (future)

use bentley_core::view::View;
use bentley_core::context::ViewContext;

pub struct LyricFollowerView {
    // TODO: Add view state
}

impl LyricFollowerView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for LyricFollowerView {
    fn on_enter(&mut self, _context: &mut ViewContext) {
        // TODO: Future implementation
    }

    fn on_exit(&mut self) {
        // TODO: Future implementation
    }

    fn update(&mut self, _dt: f32, _context: &ViewContext) {
        // TODO: Future implementation
    }
}

