use pollster::FutureExt;
use std::rc::Rc;
use wgpu::*;
use winit::window::Window;

pub struct Renderer {
    pub instance: Instance,
    pub window: Rc<Window>,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
    pub depth_texture: Texture,
}
impl Renderer {
    pub const DEPTH_TEXTURE_FORMAT: TextureFormat = TextureFormat::Depth32Float;
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
        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth texture"),
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            format: Self::DEPTH_TEXTURE_FORMAT,
            view_formats: &[],
            size: Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            mip_level_count: 1,
            sample_count: 1,
        });

        Self {
            instance,
            surface,
            surface_config,
            device,
            queue,
            window,
            depth_texture,
        }
    }
    pub fn render(&self, on_render_pass: impl FnOnce(&mut RenderPass)) {
        self.surface.configure(&self.device, &self.surface_config);
        
        let surface_texture = self.surface.get_current_texture().unwrap();
        let view = surface_texture.texture.create_view(&Default::default());
        
        let depth_view = self.depth_texture.create_view(&Default::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("command_encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("begin_render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            on_render_pass(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }
}
