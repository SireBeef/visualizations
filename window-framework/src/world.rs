use crate::canvas::{Canvas, CoordinateSystem};
use crate::input::InputState;

/// Configuration for a World implementation
pub struct WorldConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub coordinate_system: CoordinateSystem,
}

impl WorldConfig {
    /// Create a new WorldConfig
    pub fn new(width: u32, height: u32, title: impl Into<String>, coordinate_system: CoordinateSystem) -> Self {
        Self {
            width,
            height,
            title: title.into(),
            coordinate_system,
        }
    }
}

/// Trait that defines the interface for a visualization world
pub trait World: Sized {
    /// Create a new instance of the world
    fn new() -> Self;

    /// Get the configuration for this world (window size, title, etc.)
    fn config() -> WorldConfig;

    /// Update the world state (called once per frame)
    fn update(&mut self);

    /// Draw the world state to the canvas
    fn draw(&self, canvas: &mut Canvas);

    /// Handle input events (called once per frame before update)
    ///
    /// Default implementation does nothing
    fn handle_input(&mut self, _input: &InputState) {}
}
