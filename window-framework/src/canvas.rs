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
    physical_width: u32,
    physical_height: u32,
    logical_width: u32,
    logical_height: u32,
    pixel_scale_x: u32,
    pixel_scale_y: u32,
    coordinate_system: CoordinateSystem,
    show_grid: bool,
    grid_color: (u8, u8, u8, u8),
}

impl<'a> Canvas<'a> {
    /// Create a new canvas wrapping a frame buffer
    pub fn new(
        frame: &'a mut [u8],
        physical_width: u32,
        physical_height: u32,
        logical_width: u32,
        logical_height: u32,
        coordinate_system: CoordinateSystem,
        show_grid: bool,
        grid_color: (u8, u8, u8, u8),
    ) -> Self {
        let pixel_scale_x = physical_width / logical_width;
        let pixel_scale_y = physical_height / logical_height;

        Self {
            frame,
            physical_width,
            physical_height,
            logical_width,
            logical_height,
            pixel_scale_x,
            pixel_scale_y,
            coordinate_system,
            show_grid,
            grid_color,
        }
    }

    /// Get the logical width of the canvas (in logical pixels)
    pub fn width(&self) -> u32 {
        self.logical_width
    }

    /// Get the logical height of the canvas (in logical pixels)
    pub fn height(&self) -> u32 {
        self.logical_height
    }

    /// Get the coordinate system of the canvas
    pub fn coordinate_system(&self) -> CoordinateSystem {
        self.coordinate_system
    }

    /// Convert user coordinates to logical buffer coordinates
    fn to_logical_coords(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        let (logical_x, logical_y) = match self.coordinate_system {
            CoordinateSystem::TopLeft => (x, y),
            CoordinateSystem::Center => {
                let center_x = (self.logical_width / 2) as i32;
                let center_y = (self.logical_height / 2) as i32;
                (x + center_x, center_y - y)
            }
        };

        // Check bounds
        if logical_x >= 0 && logical_x < self.logical_width as i32
            && logical_y >= 0 && logical_y < self.logical_height as i32 {
            Some((logical_x as u32, logical_y as u32))
        } else {
            None
        }
    }

    /// Set a physical pixel in the frame buffer
    fn set_physical_pixel(&mut self, phys_x: u32, phys_y: u32, color: (u8, u8, u8, u8)) {
        if phys_x < self.physical_width && phys_y < self.physical_height {
            let idx = ((phys_y * self.physical_width + phys_x) * 4) as usize;
            self.frame[idx] = color.0;
            self.frame[idx + 1] = color.1;
            self.frame[idx + 2] = color.2;
            self.frame[idx + 3] = color.3;
        }
    }

    /// Get a physical pixel from the frame buffer
    fn get_physical_pixel(&self, phys_x: u32, phys_y: u32) -> Option<(u8, u8, u8, u8)> {
        if phys_x < self.physical_width && phys_y < self.physical_height {
            let idx = ((phys_y * self.physical_width + phys_x) * 4) as usize;
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

    /// Set a logical pixel at the given coordinates with the specified color
    ///
    /// This will fill the corresponding block of physical pixels
    ///
    /// Color format: (R, G, B, A) where each component is 0-255
    ///
    /// Returns true if the pixel was set, false if out of bounds
    pub fn set_pixel(&mut self, x: i32, y: i32, color: (u8, u8, u8, u8)) -> bool {
        if let Some((logical_x, logical_y)) = self.to_logical_coords(x, y) {
            // Calculate the top-left physical pixel for this logical pixel
            let phys_x_start = logical_x * self.pixel_scale_x;
            let phys_y_start = logical_y * self.pixel_scale_y;

            // Fill the block of physical pixels
            for dy in 0..self.pixel_scale_y {
                for dx in 0..self.pixel_scale_x {
                    self.set_physical_pixel(phys_x_start + dx, phys_y_start + dy, color);
                }
            }
            true
        } else {
            false
        }
    }

    /// Get the color of a logical pixel at the given coordinates
    ///
    /// Returns the color of the top-left physical pixel in the logical pixel block
    ///
    /// Returns None if the coordinates are out of bounds
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<(u8, u8, u8, u8)> {
        if let Some((logical_x, logical_y)) = self.to_logical_coords(x, y) {
            let phys_x = logical_x * self.pixel_scale_x;
            let phys_y = logical_y * self.pixel_scale_y;
            self.get_physical_pixel(phys_x, phys_y)
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

    /// Draw grid lines between logical pixels (internal method, called automatically if show_grid is true)
    pub(crate) fn draw_grid(&mut self) {
        if !self.show_grid || self.pixel_scale_x <= 1 || self.pixel_scale_y <= 1 {
            return;
        }

        // Draw vertical lines
        for logical_x in 0..=self.logical_width {
            let phys_x = logical_x * self.pixel_scale_x;
            if phys_x < self.physical_width {
                for phys_y in 0..self.physical_height {
                    self.set_physical_pixel(phys_x, phys_y, self.grid_color);
                }
            }
        }

        // Draw horizontal lines
        for logical_y in 0..=self.logical_height {
            let phys_y = logical_y * self.pixel_scale_y;
            if phys_y < self.physical_height {
                for phys_x in 0..self.physical_width {
                    self.set_physical_pixel(phys_x, phys_y, self.grid_color);
                }
            }
        }
    }
}
