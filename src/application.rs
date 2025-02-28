use crate::mesh::Mesh;
use crate::mesh_pipeline::MeshPipeline;
use crate::renderer::Renderer;
use cgmath::vec3;
use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

pub struct Application {
    window: Rc<Window>,
    renderer: Renderer,
    mesh_pipeline: MeshPipeline,
    meshes: Vec<Mesh>,
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
        let mesh_pipeline = MeshPipeline::new(&renderer);
        let meshes = vec![Mesh::from_arrays(
            &renderer,
            &[
                vec3(-0.5, 0.5, 0.0),
                vec3(-0.5, -0.5, 0.0),
                vec3(0.5, 0.5, 0.0),
                vec3(0.5, 0.5, 0.0),
                vec3(-0.5, -0.5, 0.0),
                vec3(0.5, -0.5, 0.0),
            ],
            None,
        )];
        Self {
            renderer,
            window,
            mesh_pipeline,
            meshes,
        }
    }
    pub fn window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                self.renderer.render(|render_pass| {
                    self.mesh_pipeline.prepare(render_pass);
                    for mesh in self.meshes.iter() {
                        mesh.render(render_pass);
                    }
                });

                self.window.request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
