use wgpu::*;
use crate::renderer::Renderer;

pub struct DepthTexture {
    pub internal: Texture,
    pub view: TextureView,
}
impl DepthTexture {
    pub const FORMAT: TextureFormat = TextureFormat::Depth32Float;
    pub fn new(renderer: &Renderer) -> Self {
        let internal = renderer.device.create_texture(&TextureDescriptor {
            label: Some("Depth texture"),
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            format: Self::FORMAT,
            view_formats: &[],
            size: Extent3d {
                width: renderer.surface_config.width,
                height: renderer.surface_config.height,
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            mip_level_count: 1,
            sample_count: 1,
        });
        let view = internal.create_view(&TextureViewDescriptor::default());
        Self { internal, view }
    }
}