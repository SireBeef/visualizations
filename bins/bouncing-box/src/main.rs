use window_framework::{Canvas, CoordinateSystem, World, WorldConfig};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

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
        WorldConfig::new(WIDTH, HEIGHT, "Bouncing Box", CoordinateSystem::TopLeft)
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
        canvas.clear((0x48, 0xb2, 0xe8, 0xff));

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
