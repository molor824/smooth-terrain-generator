use pollster::FutureExt;
use std::rc::Rc;
use wgpu::*;
use winit::window::Window;

pub struct Renderer {
    instance: Instance,
    window: Rc<Window>,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
}
impl Renderer {
    pub fn from_window(window: Rc<Window>) -> Self {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::VULKAN,
            flags: InstanceFlags::debugging(),
            backend_options: Default::default(),
        });
        // SAFETY: Surface must be dropped before window
        let surface = instance
            .create_surface(unsafe { std::mem::transmute::<&Window, &'static Window>(&window) })
            .unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("request_device"),
                    ..Default::default()
                },
                None,
            )
            .block_on()
            .unwrap();
        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities
            .formats
            .iter()
            .cloned()
            .find(|format| format.is_srgb())
            .unwrap_or(capabilities.formats[0]);
        let surface_config = SurfaceConfiguration {
            alpha_mode: capabilities.alpha_modes[0],
            present_mode: PresentMode::Fifo,
            format: surface_format,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            usage: TextureUsages::RENDER_ATTACHMENT,
            width: window.inner_size().width,
            height: window.inner_size().height,
        };

        Self {
            instance,
            surface,
            surface_config,
            device,
            queue,
            window,
        }
    }
    pub fn render(&self) {
        self.surface.configure(&self.device, &self.surface_config);
        
        let surface_texture = self.surface.get_current_texture().unwrap();
        let view = surface_texture.texture.create_view(&Default::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("command_encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("begin_render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }
}
