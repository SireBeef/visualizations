use crate::canvas::{Canvas, CoordinateSystem};
use crate::input::InputState;

/// Configuration for a World implementation
pub struct WorldConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub coordinate_system: CoordinateSystem,
    pub pixel_grid_width: u32,
    pub pixel_grid_height: u32,
    pub show_grid: bool,
    pub grid_color: (u8, u8, u8, u8),
}

impl WorldConfig {
    /// Create a new WorldConfig
    ///
    /// If pixel_grid_width or pixel_grid_height are None, they default to width and height respectively (1:1 pixel mapping)
    /// If show_grid is true, grid lines will be drawn between logical pixels (only visible when pixel grid is smaller than canvas)
    pub fn new(
        width: u32,
        height: u32,
        title: impl Into<String>,
        coordinate_system: CoordinateSystem,
        pixel_grid_width: Option<u32>,
        pixel_grid_height: Option<u32>,
        show_grid: bool,
        grid_color: (u8, u8, u8, u8),
    ) -> Self {
        Self {
            width,
            height,
            title: title.into(),
            coordinate_system,
            pixel_grid_width: pixel_grid_width.unwrap_or(width),
            pixel_grid_height: pixel_grid_height.unwrap_or(height),
            show_grid,
            grid_color,
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
