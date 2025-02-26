use crate::renderer::Renderer;
use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

pub struct Application {
    window: Rc<Window>,
    renderer: Renderer,
}
impl Application {
    pub fn from_event_loop(event_loop: &ActiveEventLoop) -> Self {
        let window = Rc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_inner_size(PhysicalSize::new(800.0, 600.0))
                        .with_title("Smooth terrain"),
                )
                .unwrap(),
        );
        let renderer = Renderer::from_window(Rc::clone(&window));
        Self { renderer, window }
    }
    pub fn window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                self.renderer.render();

                self.window.request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
