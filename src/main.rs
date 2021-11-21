use paintings::{
    app::{run, App, AppState},
    renderer::Layer,
};

struct State {
    layer: Layer,
}

impl State {
    fn new() -> Self {
        State {
            layer: Layer::new(Some(wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            })),
        }
    }
}

impl AppState for State {
    fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(
        &mut self,
        renderer: &paintings::renderer::Renderer,
    ) -> Result<(), wgpu::SurfaceError> {
        self.layer.render(renderer)
    }
}

fn main() {
    let app = pollster::block_on(App::new());
    let state = State::new();
    run(app, state);
}
