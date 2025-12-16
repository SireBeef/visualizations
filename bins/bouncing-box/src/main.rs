use window_framework::{Canvas, CoordinateSystem, World, WorldConfig};

const RESOLUTION_WIDTH: u32 = 320;
const RESOLUTION_HEIGHT: u32 = 240;
const WIDTH: u32 = RESOLUTION_WIDTH / 8;
const HEIGHT: u32 = RESOLUTION_HEIGHT / 8;
const BOX_SIZE: i16 = 8;

struct BouncingBox {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl World for BouncingBox {
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    fn config() -> WorldConfig {
        WorldConfig::new(
            RESOLUTION_WIDTH,
            RESOLUTION_HEIGHT,
            "Bouncing Box",
            CoordinateSystem::TopLeft,
            Some(WIDTH),
            Some(HEIGHT),
            true,
            (255, 255, 255, 255),
        )
    }

    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    fn draw(&self, canvas: &mut Canvas) {
        // Clear with cyan background
        canvas.clear((0, 0, 0, 0));

        // Draw purple box
        canvas.fill_rect(
            self.box_x as i32,
            self.box_y as i32,
            BOX_SIZE as u32,
            BOX_SIZE as u32,
            (0x5e, 0x48, 0xe8, 0xff),
        );
    }
}

fn main() {
    window_framework::run::<BouncingBox>().unwrap();
}
