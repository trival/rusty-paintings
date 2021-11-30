use paintings::{
    app::{run, App, AppState},
    renderer::Layer,
};
use winit::window::Window;

struct State {
    layer: Layer,
}

impl State {
    fn new() -> Self {
        State {
            layer: Layer::new().with_clear_color(Some(wgpu::Color::default())),
        }
    }
}

impl AppState for State {
    fn input(&mut self, event: &winit::event::WindowEvent, window: &Window) -> bool {
        if let winit::event::WindowEvent::CursorMoved { position, .. } = event {
            let s = window.inner_size();
            self.layer.set_clear_color(Some(wgpu::Color {
                r: position.x / s.width as f64,
                g: position.y / s.height as f64,
                ..Default::default()
            }));
            // println!("cursor position: {:?}", position);
            // println!("Window size: {:?}", s);
        }

        false
    }

    fn update(&mut self, _window: &Window) {}

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
