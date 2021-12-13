use paintings::prelude::*;
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
    layer_idx: usize,
}

impl AppView<State> for View {
    fn init(renderer: &mut Renderer, state: &State) -> Self {
        let layer_idx = renderer.make_layer(vec![]);
        renderer
            .layer_mut(layer_idx)
            .set_clear_color(Some(state.color));
        Self { layer_idx }
    }

    fn resize(&mut self, _window: &Window) {}

    fn render(
        &mut self,
        renderer: &mut Renderer,
        _state: &State,
    ) -> Result<(), wgpu::SurfaceError> {
        renderer.render_layer(self.layer_idx)
    }
}

fn main() {
    let app = pollster::block_on(App::new(WindowBuilder::new()));
    run::<State, View>(app);
}
