use paintings::{
    app::{run, App, AppState, AppView},
    renderer::{Form, Layer, Renderer, Shade, Sketch},
};
use winit::window::{Window, WindowBuilder};

struct State {
    color: wgpu::Color,
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
        }
    }

    fn input(&mut self, _event: &winit::event::WindowEvent, _window: &Window) -> bool {
        false
    }

    fn update(&mut self, _window: &Window) {}
}

struct View {
    sketches: Vec<Sketch>,
    layer: Layer,
}

impl AppView<State> for View {
    fn init(state: &State, renderer: &Renderer) -> Self {
        Self {
            layer: Layer::new().with_clear_color(Some(state.color)),
            sketches: vec![Sketch::new(
                renderer,
                &Shade::new(renderer, include_str!("shader.wgsl")),
                &Form::new(3),
            )],
        }
    }

    fn render(
        &mut self,
        renderer: &paintings::renderer::Renderer,
        _state: &State,
    ) -> Result<(), wgpu::SurfaceError> {
        self.layer.render(renderer, &self.sketches)
    }

    fn resize(&mut self, _window: &Window) {}
}

fn main() {
    let app = pollster::block_on(App::new(WindowBuilder::new()));
    run::<State, View>(app);
}
