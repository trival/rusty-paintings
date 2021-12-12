use paintings::prelude::*;
use winit::window::{Window, WindowBuilder};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

struct State {
    color: wgpu::Color,
    vertices: Vec<Vertex>,
}

impl AppState for State {
    fn init() -> Self {
        Self {
            color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            vertices: vec![
                Vertex {
                    position: [0.0, 0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
            ],
        }
    }

    fn input(&mut self, _event: &winit::event::WindowEvent, _window: &Window) -> bool {
        false
    }

    fn update(&mut self, _window: &Window) {}
}

struct View {
    layer: Layer,
}

impl AppView<State> for View {
    fn init(state: &State, _renderer: &Renderer) -> Self {
        Self {
            layer: Layer::new().with_clear_color(Some(state.color)),
        }
    }

    fn render(
        &mut self,
        renderer: &paintings::renderer::Renderer,
        _state: &State,
    ) -> Result<(), wgpu::SurfaceError> {
        self.layer.render(renderer, &[])
    }

    fn resize(&mut self, _window: &Window) {}
}

fn main() {
    let app = pollster::block_on(App::new(WindowBuilder::new()));
    run::<State, View>(app);
}
