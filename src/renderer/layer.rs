use super::{sketch::Sketch, Renderer};

#[derive(Debug, Clone, Copy)]
pub struct Layer {
    clear_color: Option<wgpu::Color>,
}

impl Layer {
    pub fn new() -> Self {
        Layer { clear_color: None }
    }

    pub fn with_clear_color(mut self, clear_color: Option<wgpu::Color>) -> Self {
        self.clear_color = clear_color;
        self
    }

    pub fn set_clear_color(&mut self, clear_color: Option<wgpu::Color>) -> &Self {
        self.clear_color = clear_color;
        self
    }

    pub fn render(
        &mut self,
        renderer: &Renderer,
        sketches: &[Sketch],
    ) -> Result<(), wgpu::SurfaceError> {
        let output = renderer.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = renderer.get_encoder();

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
            for sketch in sketches {
                render_pass.set_pipeline(&sketch.pipeline); // 2.
                render_pass.draw(0..sketch.vertex_count, 0..1); // 3.
            }
        }

        // submit will accept anything that implements IntoIter
        renderer.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
