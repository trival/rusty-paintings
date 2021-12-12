use super::Renderer;

pub struct Shade {
    pub shader: wgpu::ShaderModule,
}

impl Shade {
    pub fn new(renderer: &Renderer, shader_source: &'static str) -> Self {
        Self {
            shader: renderer
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("Shader"),
                    source: wgpu::ShaderSource::Wgsl(shader_source.into()),
                }),
        }
    }
}
