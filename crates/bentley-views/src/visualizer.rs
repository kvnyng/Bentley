//! Audio Visualizer View (future)

use bentley_core::view::View;
use bentley_core::context::ViewContext;

pub struct VisualizerView {
    // TODO: Add view state
}

impl VisualizerView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for VisualizerView {
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

