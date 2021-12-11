use paintings::{
    app::{run, App, AppState, AppView},
    renderer::Layer,
};
use winit::window::{Window, WindowBuilder};

#[derive(Clone, Copy)]
struct State {
    color: wgpu::Color,
}

impl Default for State {
    fn default() -> Self {
        Self {
            color: wgpu::Color::default(),
        }
    }
}

impl AppState for State {
    fn input(&mut self, event: &winit::event::WindowEvent, window: &Window) -> bool {
        if let winit::event::WindowEvent::CursorMoved { position, .. } = event {
            let s = window.inner_size();
            self.color = wgpu::Color {
                r: position.x / s.width as f64,
                g: position.y / s.height as f64,
                ..Default::default()
            };
            // println!("cursor position: {:?}", position);
            // println!("Window size: {:?}", s);
        }

        false
    }

    fn update(&mut self, _window: &Window) {}
}

struct View {
    layer: Layer,
}

impl AppView<State> for View {
    fn init(_state: &State) -> Self {
        Self {
            layer: Layer::new(),
        }
    }

    fn resize(&mut self, _window: &Window) {}

    fn render(
        &mut self,
        renderer: &paintings::renderer::Renderer,
        state: &State,
    ) -> Result<(), wgpu::SurfaceError> {
        self.layer.set_clear_color(Some(state.color));
        self.layer.render(renderer, &[])
    }
}

fn main() {
    let app = pollster::block_on(App::new(WindowBuilder::new()));
    run::<State, View>(app);
}
