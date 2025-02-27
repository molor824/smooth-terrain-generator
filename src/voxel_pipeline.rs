use wgpu::*;
use crate::depth_texture::DepthTexture;
use crate::renderer::Renderer;

pub struct VoxelPipeline {
    internal: RenderPipeline,
    depth_texture: Texture,
}
impl VoxelPipeline {
    pub fn new(renderer: &Renderer) -> Self {
        let shader_module = renderer.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Voxel pipeline shader module"),
            source: unimplemented!(),
        });
        let layout = renderer.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Voxel pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let pipeline = renderer.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("VoxelPipeline"),
            primitive: PrimitiveState {
                polygon_mode: PolygonMode::Fill,
                cull_mode: Some(Face::Back),
                front_face: FrontFace::Ccw,
                topology: PrimitiveTopology::TriangleStrip,
                strip_index_format: Some(IndexFormat::Uint16),
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                bias: DepthBiasState::default(),
                stencil: StencilState::default(),
                depth_compare: CompareFunction::Less,
                format: DepthTexture::FORMAT,
                depth_write_enabled: true,
            }),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vertex_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fragment_main"),
                targets: &[],
                compilation_options: Default::default(),
            }),
            cache: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });
        Self { internal, depth_texture }
    }
}