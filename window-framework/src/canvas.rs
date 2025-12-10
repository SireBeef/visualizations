/// Coordinate system for the canvas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSystem {
    /// Origin (0, 0) is at the top-left corner
    TopLeft,
    /// Origin (0, 0) is at the center of the canvas
    Center,
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        CoordinateSystem::TopLeft
    }
}

/// A canvas for drawing pixels with configurable coordinate systems
pub struct Canvas<'a> {
    frame: &'a mut [u8],
    width: u32,
    height: u32,
    coordinate_system: CoordinateSystem,
}

impl<'a> Canvas<'a> {
    /// Create a new canvas wrapping a frame buffer
    pub fn new(
        frame: &'a mut [u8],
        width: u32,
        height: u32,
        coordinate_system: CoordinateSystem,
    ) -> Self {
        Self {
            frame,
            width,
            height,
            coordinate_system,
        }
    }

    /// Get the width of the canvas
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the canvas
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get the coordinate system of the canvas
    pub fn coordinate_system(&self) -> CoordinateSystem {
        self.coordinate_system
    }

    /// Convert user coordinates to buffer coordinates
    fn to_buffer_coords(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        let (buf_x, buf_y) = match self.coordinate_system {
            CoordinateSystem::TopLeft => (x, y),
            CoordinateSystem::Center => {
                let center_x = (self.width / 2) as i32;
                let center_y = (self.height / 2) as i32;
                (x + center_x, center_y - y)
            }
        };

        // Check bounds
        if buf_x >= 0 && buf_x < self.width as i32 && buf_y >= 0 && buf_y < self.height as i32 {
            Some((buf_x as u32, buf_y as u32))
        } else {
            None
        }
    }

    /// Set a pixel at the given coordinates with the specified color
    ///
    /// Color format: (R, G, B, A) where each component is 0-255
    ///
    /// Returns true if the pixel was set, false if out of bounds
    pub fn set_pixel(&mut self, x: i32, y: i32, color: (u8, u8, u8, u8)) -> bool {
        if let Some((buf_x, buf_y)) = self.to_buffer_coords(x, y) {
            let idx = ((buf_y * self.width + buf_x) * 4) as usize;
            self.frame[idx] = color.0;     // R
            self.frame[idx + 1] = color.1; // G
            self.frame[idx + 2] = color.2; // B
            self.frame[idx + 3] = color.3; // A
            true
        } else {
            false
        }
    }

    /// Get the color of a pixel at the given coordinates
    ///
    /// Returns None if the coordinates are out of bounds
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<(u8, u8, u8, u8)> {
        if let Some((buf_x, buf_y)) = self.to_buffer_coords(x, y) {
            let idx = ((buf_y * self.width + buf_x) * 4) as usize;
            Some((
                self.frame[idx],
                self.frame[idx + 1],
                self.frame[idx + 2],
                self.frame[idx + 3],
            ))
        } else {
            None
        }
    }

    /// Clear the entire canvas with the specified color
    pub fn clear(&mut self, color: (u8, u8, u8, u8)) {
        for chunk in self.frame.chunks_exact_mut(4) {
            chunk[0] = color.0;
            chunk[1] = color.1;
            chunk[2] = color.2;
            chunk[3] = color.3;
        }
    }

    /// Fill a rectangular region with the specified color
    ///
    /// The rectangle is defined by (x, y) as the top-left corner and (width, height) as dimensions
    /// when using TopLeft coordinates, or centered at (x, y) when using Center coordinates
    pub fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: (u8, u8, u8, u8)) {
        for dy in 0..height as i32 {
            for dx in 0..width as i32 {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
    }
}
