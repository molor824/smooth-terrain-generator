use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use crate::application::Application;

#[derive(Default)]
pub struct AppHandler(Option<Application>);
impl winit::application::ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.0 = Some(Application::from_event_loop(event_loop));
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let Some(application) = self.0.as_mut() else {
            eprintln!("Application is None!");
            return;
        };
        application.window_event(event_loop, event);
    }
}
