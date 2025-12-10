use std::sync::Arc;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{canvas::{Canvas, CoordinateSystem}, input::InputState, world::World};

pub struct App<W: World> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    world: Option<W>,
    input: InputState,
    width: u32,
    height: u32,
    coordinate_system: CoordinateSystem,
}

impl<W: World> App<W> {
    pub fn new(width: u32, height: u32, coordinate_system: CoordinateSystem) -> Self {
        Self {
            window: None,
            pixels: None,
            world: None,
            input: InputState::new(),
            width,
            height,
            coordinate_system,
        }
    }
}

impl<W: World> ApplicationHandler for App<W> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = WindowAttributes::default()
            .with_title(W::config().title);

        let window = event_loop
            .create_window(window_attrs)
            .unwrap();
        let window = Arc::new(window);
        self.window = Some(window.clone());

        self.pixels = {
            let (window_width, window_height) = window.inner_size().into();
            let surface_texture = SurfaceTexture::new(window_width, window_height, window.clone());
            match Pixels::new(self.width, self.height, surface_texture) {
                Ok(pixels) => {
                    window.request_redraw();
                    Some(pixels)
                }
                Err(err) => {
                    log_error("pixels::new", err);
                    event_loop.exit();
                    None
                }
            }
        };

        self.world = Some(W::new());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(world) = self.world.as_mut() {
                    world.handle_input(&self.input);
                    world.update();
                    let frame = self.pixels.as_mut().unwrap().frame_mut();
                    let mut canvas = Canvas::new(frame, self.width, self.height, self.coordinate_system);
                    world.draw(&mut canvas);
                    if let Err(err) = self.pixels.as_ref().unwrap().render() {
                        log_error("pixels.render", err);
                        event_loop.exit();
                    }
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let Err(err) = self
                    .pixels
                    .as_mut()
                    .unwrap()
                    .resize_surface(size.width, size.height)
                {
                    log_error("pixels.resize_surface", err);
                    event_loop.exit()
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if key_code == KeyCode::Escape {
                        event_loop.exit();
                    }

                    if event.state.is_pressed() {
                        self.input.keys_pressed.insert(key_code);
                    } else {
                        self.input.keys_pressed.remove(&key_code);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input.mouse_position = Some((position.x, position.y));
            }
            WindowEvent::CursorLeft { .. } => {
                self.input.mouse_position = None;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state.is_pressed();
                match button {
                    winit::event::MouseButton::Left => self.input.mouse_buttons.0 = pressed,
                    winit::event::MouseButton::Middle => self.input.mouse_buttons.1 = pressed,
                    winit::event::MouseButton::Right => self.input.mouse_buttons.2 = pressed,
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
