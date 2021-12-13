use crate::prelude::Renderer;
use bytemuck::Pod;
use wgpu::util::DeviceExt;

pub struct VertexBuffer {
    pub vertices: wgpu::Buffer,
    pub vertex_count: u32,
    pub array_stride: u64,
    pub attributes: Vec<wgpu::VertexAttribute>,
}

impl VertexBuffer {
    pub fn new<T: Pod>(
        renderer: &Renderer,
        vertex_data: &[T],
        attributes: &[wgpu::VertexAttribute],
    ) -> Self {
        Self {
            vertices: renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("vertex buffer label (TODO)"),
                    contents: bytemuck::cast_slice(vertex_data),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            vertex_count: vertex_data.len() as u32,
            array_stride: std::mem::size_of::<T>() as wgpu::BufferAddress,
            attributes: attributes.to_vec(),
        }
    }
}

pub enum Form {
    SimpleRange { vertex_count: u32 },
    Vertices(VertexBuffer),
}
