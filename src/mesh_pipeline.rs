use crate::depth_texture::DepthTexture;
use crate::renderer::Renderer;
use wgpu::*;

pub struct MeshPipeline {
    pub internal: RenderPipeline,
}
impl MeshPipeline {
    pub fn new(renderer: &Renderer) -> Self {
        let shader_module = renderer
            .device
            .create_shader_module(include_wgsl!("shaders/mesh.wgsl"));
        let layout = renderer
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Mesh pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let pipeline = renderer
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Mesh pipeline"),
                vertex: VertexState {
                    buffers: &[VertexBufferLayout {
                        attributes: &vertex_attr_array![0 => Float32x3],
                        array_stride: 0,
                        step_mode: VertexStepMode::Vertex,
                    }],
                    entry_point: Some("vertex_main"),
                    module: &shader_module,
                    compilation_options: Default::default(),
                },
                layout: Some(&layout),
                cache: None,
                depth_stencil: Some(DepthStencilState {
                    format: DepthTexture::FORMAT,
                    depth_compare: CompareFunction::Less,
                    depth_write_enabled: true,
                    bias: Default::default(),
                    stencil: Default::default(),
                }),
                fragment: Some(FragmentState {
                    module: &shader_module,
                    compilation_options: Default::default(),
                    entry_point: Some("fragment_main"),
                    targets: &[Some(ColorTargetState {
                        format: renderer.surface_config.format,
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::all(),
                    })],
                }),
                multisample: MultisampleState::default(),
                multiview: None,
                primitive: PrimitiveState {
                    conservative: false,
                    cull_mode: Some(Face::Back),
                    front_face: FrontFace::Ccw,
                    polygon_mode: PolygonMode::Fill,
                    strip_index_format: Some(IndexFormat::Uint16),
                    topology: PrimitiveTopology::TriangleList,
                    unclipped_depth: false,
                },
            });

        Self { internal: pipeline }
    }
    pub fn prepare(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.internal);
    }
}
