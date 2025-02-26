use winit::event_loop::{ControlFlow, EventLoop};
use crate::app_handler::AppHandler;

mod app_handler;
mod application;
mod renderer;

fn main() {
    let mut app_handler = AppHandler::default();
    let event_loop = EventLoop::new().unwrap();
    
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app_handler).unwrap();
}