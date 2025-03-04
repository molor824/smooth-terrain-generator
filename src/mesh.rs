use crate::mesh_pipeline::MeshPipeline;
use crate::renderer::Renderer;
use bytemuck::{bytes_of, cast_slice};
use wgpu::util::*;
use wgpu::*;

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Option<Buffer>,
    pub color_bind_group: BindGroup,
    pub vertex_count: usize,
    pub color_buffer: Buffer,
}
impl Mesh {
    pub fn from_arrays(
        renderer: &Renderer,
        mesh_pipeline: &MeshPipeline,
        vertices: &[[f32; 3]],
        indices: Option<&[u16]>,
        color: [f32; 3],
    ) -> Mesh {
        let vertex_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            usage: BufferUsages::VERTEX,
            contents: cast_slice(vertices),
        });
        let index_buffer = indices.map(|indices| {
            renderer.device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Index Buffer"),
                usage: BufferUsages::INDEX,
                contents: cast_slice(indices),
            })
        });
        let color_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Color buffer"),
            contents: bytes_of(&color),
            usage: BufferUsages::UNIFORM,
        });
        let color_bind_group = renderer.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Mesh color bind group"),
            layout: &mesh_pipeline.color_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &color_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });
        Mesh {
            vertex_buffer,
            index_buffer,
            vertex_count: indices
                .map(|indices| indices.len())
                .unwrap_or_else(|| vertices.len()),
            color_bind_group,
            color_buffer,
        }
    }
    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_bind_group(0, &self.color_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        if let Some(index_buffer) = &self.index_buffer {
            render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..(self.vertex_count as u32), 0, 0..1);
        } else {
            render_pass.draw(0..(self.vertex_count as u32), 0..1);
        }
    }
}
