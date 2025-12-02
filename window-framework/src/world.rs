use crate::input::InputState;

/// Configuration for a World implementation
pub struct WorldConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

/// Trait that defines the interface for a visualization world
pub trait World: Sized {
    /// Create a new instance of the world
    fn new() -> Self;

    /// Get the configuration for this world (window size, title, etc.)
    fn config() -> WorldConfig;

    /// Update the world state (called once per frame)
    fn update(&mut self);

    /// Draw the world state to the frame buffer
    ///
    /// Frame format: RGBA8 (4 bytes per pixel)
    fn draw(&self, frame: &mut [u8]);

    /// Handle input events (called once per frame before update)
    ///
    /// Default implementation does nothing
    fn handle_input(&mut self, _input: &InputState) {}
}
