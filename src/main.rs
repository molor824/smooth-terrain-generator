use crate::app_handler::AppHandler;
use winit::event_loop::{ControlFlow, EventLoop};

mod app_handler;
mod application;
mod renderer;
mod mesh_pipeline;
mod mesh;
mod bytes;

fn main() {
    let mut app_handler = AppHandler::default();
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app_handler).unwrap();
}
