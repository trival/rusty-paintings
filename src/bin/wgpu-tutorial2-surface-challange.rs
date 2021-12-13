use paintings::prelude::*;
use winit::window::{Window, WindowBuilder};

struct State {
    color: wgpu::Color,
}

impl AppState for State {
    fn init() -> Self {
        Self {
            color: wgpu::Color::default(),
        }
    }

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
    layer_idx: usize,
}

impl AppView<State> for View {
    fn init(renderer: &mut Renderer, _state: &State) -> Self {
        Self {
            layer_idx: renderer.make_layer(vec![]),
        }
    }

    fn resize(&mut self, _window: &Window) {}

    fn render(&mut self, renderer: &mut Renderer, state: &State) -> Result<(), wgpu::SurfaceError> {
        renderer
            .layer_mut(self.layer_idx)
            .set_clear_color(Some(state.color));
        renderer.render_layer(self.layer_idx)
    }
}

fn main() {
    let app = pollster::block_on(App::new(WindowBuilder::new()));
    run::<State, View>(app);
}
