use cgmath::Vector3;
use wgpu::*;
use wgpu::util::*;
use crate::bytes::ToBytes;
use crate::renderer::Renderer;

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Option<Buffer>,
}
impl Mesh {
    pub const fn from_buffers(vertex_buffer: Buffer, index_buffer: Option<Buffer>) -> Mesh {
        Mesh { vertex_buffer, index_buffer }
    }
    pub fn from_arrays(renderer: &Renderer, vertices: &[Vector3<f32>], indices: Option<&[u16]>) -> Mesh {
        let vertex_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            usage: BufferUsages::VERTEX,
            contents: vertices.to_bytes(),
        });
        let index_buffer = indices.map(|indices| renderer.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            usage: BufferUsages::INDEX,
            contents: indices.to_bytes(),
        }));
        Mesh { vertex_buffer, index_buffer }
    }
}