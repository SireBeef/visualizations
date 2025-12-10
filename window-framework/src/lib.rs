pub mod app;
pub mod canvas;
pub mod input;
pub mod world;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    error::EventLoopError,
};

pub use app::App;
pub use canvas::{Canvas, CoordinateSystem};
pub use input::InputState;
pub use world::{World, WorldConfig};

/// Run a visualization with the given World implementation
pub fn run<W: World + 'static>() -> Result<(), EventLoopError> {
    let config = W::config();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::<W>::new(
        config.width,
        config.height,
        config.pixel_grid_width,
        config.pixel_grid_height,
        config.coordinate_system,
        config.show_grid,
        config.grid_color,
    );
    event_loop.run_app(&mut app)
}
