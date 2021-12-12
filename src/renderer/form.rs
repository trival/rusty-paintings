use crate::prelude::Renderer;
use bytemuck::Pod;
use wgpu::util::DeviceExt;

trait VertexData: Sized {
    fn desc<'a>(attr_formats: &[wgpu::VertexFormat]) -> wgpu::VertexBufferLayout<'a> {
        let (attribs, _) = attr_formats.into_iter().enumerate().fold(
            (Vec::new(), 0 as wgpu::BufferAddress),
            |(attribs, offset), (index, format)| {
                attribs.push(wgpu::VertexAttribute {
                    format: *format,
                    shader_location: index as u32,
                    offset,
                });
                (attribs, offset + wgpu::VertexFormat::size(format))
            },
        );

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: attribs.as_slice(),
        }
    }
}

pub struct VertexBuffer<'a> {
    vertices: wgpu::Buffer,
    description: wgpu::VertexBufferLayout<'a>,
    vertex_count: u32,
}

impl VertexBuffer<'_> {
    fn new<T: VertexData + Pod>(
        renderer: &Renderer,
        vertex_data: &[T],
        attr_formats: &[wgpu::VertexFormat],
    ) -> Self {
        Self {
            vertices: renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(vertex_data),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            description: T::desc(attr_formats),
            vertex_count: vertex_data.len() as u32,
        }
    }
}

pub enum Form<'a> {
    SimpleRange { vertex_count: u32 },
    Vertices(VertexBuffer<'a>),
}
