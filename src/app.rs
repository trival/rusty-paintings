use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl From<PhysicalSize<u32>> for Dimensions {
    fn from(s: PhysicalSize<u32>) -> Self {
        Dimensions {
            width: s.width,
            height: s.height,
        }
    }
}

pub struct App {
    pub size: Dimensions,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl App {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window, size: Dimensions) -> Self {
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
        }
    }

    pub fn resize(&mut self, new_size: Dimensions) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = self.size.width;
            self.config.height = self.size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
