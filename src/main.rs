use std::{rc::Rc, time::Instant};

use mesh::Mesh;
use mesh_pipeline::MeshPipeline;
use renderer::Renderer;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod mesh;
mod mesh_pipeline;
mod renderer;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Terrain generator")
            .with_inner_size(LogicalSize::new(1280, 720))
            .build(&event_loop)
            .unwrap(),
    );
    let mut renderer = Renderer::from_window(window.clone());
    let mesh_pipeline = MeshPipeline::new(&renderer);
    let mesh1 = Mesh::from_arrays(
        &renderer,
        &mesh_pipeline,
        &[[0.0, 0.5, 0.5], [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5]],
        None,
        [1.0, 0.0, 0.0],
    );
    let mesh2 = Mesh::from_arrays(
        &renderer,
        &mesh_pipeline,
        &[[0.0, 0.5, 0.5], [-0.5, -0.5, 0.6], [0.5, -0.5, 0.4]],
        None,
        [0.0, 1.0, 0.0],
    );

    event_loop.set_control_flow(ControlFlow::Poll);

    let instant = Instant::now();
    let mut duration = instant.elapsed();

    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    target.exit();
                }
                WindowEvent::Resized(size) => renderer.resize(size.width, size.height),
                WindowEvent::RedrawRequested => {
                    window.request_redraw();

                    let last_duration = duration;
                    duration = instant.elapsed();

                    let delta_duration = duration - last_duration;
                    println!("Delta: {}ms", delta_duration.as_millis());

                    renderer.render(|render_pass| {
                        mesh_pipeline.prepare(render_pass);
                        mesh1.render(render_pass);
                        mesh2.render(render_pass);
                    });
                }
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
