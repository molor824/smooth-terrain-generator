use crate::bytes::ToBytes;
use crate::renderer::Renderer;
use cgmath::Vector3;
use wgpu::util::*;
use wgpu::*;

pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Option<Buffer>,
    vertex_count: usize,
}
impl Mesh {
    pub const fn from_buffers(
        vertex_buffer: Buffer,
        index_buffer: Option<Buffer>,
        vertex_count: usize,
    ) -> Mesh {
        Mesh {
            vertex_buffer,
            index_buffer,
            vertex_count,
        }
    }
    pub fn from_arrays(
        renderer: &Renderer,
        vertices: &[Vector3<f32>],
        indices: Option<&[u16]>,
    ) -> Mesh {
        let vertex_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            usage: BufferUsages::VERTEX,
            contents: vertices.to_bytes(),
        });
        let index_buffer = indices.map(|indices| {
            renderer.device.create_buffer_init(&BufferInitDescriptor {
                label: Some("Index Buffer"),
                usage: BufferUsages::INDEX,
                contents: indices.to_bytes(),
            })
        });
        Mesh {
            vertex_buffer,
            index_buffer,
            vertex_count: indices.map(|indices| indices.len()).unwrap_or_else(|| vertices.len()),
        }
    }
    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        if let Some(index_buffer) = &self.index_buffer {
            render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..(self.vertex_count as u32), 0, 0..1);
        } else {
            render_pass.draw(0..(self.vertex_count as u32), 0..1);
        }
    }
}
