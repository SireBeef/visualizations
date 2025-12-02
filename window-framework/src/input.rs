use std::collections::HashSet;
use winit::keyboard::KeyCode;

/// Tracks the current input state
#[derive(Default)]
pub struct InputState {
    /// Set of keys currently pressed
    pub keys_pressed: HashSet<KeyCode>,

    /// Mouse position in window coordinates (None if outside window)
    pub mouse_position: Option<(f64, f64)>,

    /// Mouse buttons currently pressed (left, middle, right)
    pub mouse_buttons: (bool, bool, bool),
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a specific key is currently pressed
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Check if left mouse button is pressed
    pub fn is_left_mouse_pressed(&self) -> bool {
        self.mouse_buttons.0
    }

    /// Check if middle mouse button is pressed
    pub fn is_middle_mouse_pressed(&self) -> bool {
        self.mouse_buttons.1
    }

    /// Check if right mouse button is pressed
    pub fn is_right_mouse_pressed(&self) -> bool {
        self.mouse_buttons.2
    }
}
