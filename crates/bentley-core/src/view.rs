//! View trait and related types

use crate::context::ViewContext;

/// Trait that all Bentley views must implement
/// 
/// Note: The render method uses generic types to avoid wgpu dependency in core
pub trait View {
    /// Called when the view becomes active
    fn on_enter(&mut self, context: &mut ViewContext);
    
    /// Called when the view is deactivated
    fn on_exit(&mut self);
    
    /// Update the view state
    /// 
    /// # Arguments
    /// * `dt` - Delta time in seconds since last update
    /// * `context` - Shared view context with services and resources
    fn update(&mut self, dt: f32, context: &ViewContext);
}

