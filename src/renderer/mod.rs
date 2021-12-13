use bytemuck::Pod;
use winit::{dpi::PhysicalSize, window::Window};

mod form;
mod layer;
mod shade;
mod sketch;

pub mod prelude {
    pub use super::form::*;
    pub use super::layer::*;
    pub use super::shade::*;
    pub use super::sketch::*;
    pub use super::*;
}
use prelude::*;

pub struct Renderer {
    pub size: PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,

    shades: Vec<Shade>,
    forms: Vec<Form>,
    sketches: Vec<Sketch>,
    layers: Vec<Layer>,
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Renderer {
        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let size = window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            shades: vec![],
            forms: vec![],
            sketches: vec![],
            layers: vec![],
        }
    }

    pub fn resize(&mut self, window: &Window) {
        let new_size = window.inner_size();
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = self.size.width;
            self.config.height = self.size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn make_form_simple_range(&mut self, vertex_count: u32) -> usize {
        let form = Form::SimpleRange { vertex_count };
        self.forms.push(form);
        self.forms.len() - 1
    }

    pub fn make_form_vertices<T: Pod>(
        &mut self,
        vertex_data: &[T],
        attributes: &[wgpu::VertexAttribute],
    ) -> usize {
        let form = Form::Vertices(VertexBuffer::new(self, vertex_data, attributes));
        self.forms.push(form);
        self.forms.len() - 1
    }

    pub fn make_shade(&mut self, shader_source: &'static str) -> usize {
        let shade = Shade::new(self, shader_source);
        self.shades.push(shade);
        self.shades.len() - 1
    }

    pub fn make_sketch(&mut self, shade_idx: usize, form_idx: usize) -> usize {
        let sketch = Sketch::new(self, shade_idx, form_idx);
        self.sketches.push(sketch);
        self.sketches.len() - 1
    }

    pub fn make_layer(&mut self, sketches: Vec<usize>) -> usize {
        let layer = Layer::new(sketches);
        self.layers.push(layer);
        self.layers.len() - 1
    }

    pub fn form(&self, idx: usize) -> &Form {
        self.forms.get(idx).expect("form index invalid")
    }
    pub fn shade(&self, idx: usize) -> &Shade {
        self.shades.get(idx).expect("shade index invalid")
    }
    pub fn sketch(&self, idx: usize) -> &Sketch {
        self.sketches.get(idx).expect("sketch index invalid")
    }
    pub fn layer(&self, idx: usize) -> &Layer {
        self.layers.get(idx).expect("layer index invalid")
    }

    pub fn form_mut(&mut self, idx: usize) -> &mut Form {
        self.forms.get_mut(idx).expect("form index invalid")
    }
    pub fn shade_mut(&mut self, idx: usize) -> &mut Shade {
        self.shades.get_mut(idx).expect("shade index invalid")
    }
    pub fn sketch_mut(&mut self, idx: usize) -> &mut Sketch {
        self.sketches.get_mut(idx).expect("sketch index invalid")
    }
    pub fn layer_mut(&mut self, idx: usize) -> &mut Layer {
        self.layers.get_mut(idx).expect("layer index invalid")
    }

    pub fn render_layer(&self, idx: usize) -> Result<(), wgpu::SurfaceError> {
        self.layer(idx).render(self)
    }
}
