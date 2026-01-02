//! Autonomous Tetris View (future)

use bentley_core::view::View;
use bentley_core::context::ViewContext;

pub struct TetrisView {
    // TODO: Add view state
}

impl TetrisView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for TetrisView {
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

