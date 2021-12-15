use crate::prelude::*;

pub struct Layer {
    clear_color: Option<wgpu::Color>,
    sketch_indices: Vec<usize>,
}

impl Layer {
    pub fn new(sketch_indices: Vec<usize>) -> Layer {
        Layer {
            clear_color: None,
            sketch_indices,
        }
    }

    pub fn with_clear_color(mut self, clear_color: Option<wgpu::Color>) -> Self {
        self.clear_color = clear_color;
        self
    }

    pub fn set_clear_color(&mut self, clear_color: Option<wgpu::Color>) -> &Self {
        self.clear_color = clear_color;
        self
    }

    pub fn render(&self, renderer: &Renderer) -> Result<(), wgpu::SurfaceError> {
        let output = renderer.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: match self.clear_color {
                            Some(color) => wgpu::LoadOp::Clear(color),
                            None => wgpu::LoadOp::Load,
                        },
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            for sketch in self.sketch_indices.iter() {
                let sketch = renderer.sketch(*sketch);
                let form = renderer.form(sketch.form_idx);
                render_pass.set_pipeline(&sketch.pipeline); // 2.
                match form {
                    Form::SimpleRange { vertex_count } => {
                        render_pass.draw(0..*vertex_count, 0..1); // 3.
                    }
                    Form::Vertices(VertexBuffer {
                        vertices,
                        vertex_count,
                        ..
                    }) => {
                        render_pass.set_vertex_buffer(0, vertices.slice(..));
                        render_pass.draw(0..*vertex_count, 0..1);
                    }
                    Form::IndexedVertices(VertexIndexBuffer {
                        vertices,
                        vertex_count,
                        indices,
                        ..
                    }) => {
                        render_pass.set_vertex_buffer(0, vertices.slice(..));
                        render_pass.set_index_buffer(indices.slice(..), wgpu::IndexFormat::Uint32);
                        render_pass.draw_indexed(0..*vertex_count, 0, 0..1);
                    }
                }
            }
        }

        // submit will accept anything that implements IntoIter
        renderer.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
