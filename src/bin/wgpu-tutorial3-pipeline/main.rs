use paintings::prelude::*;
use winit::{
    event::*,
    window::{Window, WindowBuilder},
};

#[derive(Debug)]
enum SelectedSketch {
    One,
    Two,
}

struct State {
    color: wgpu::Color,
    selected: SelectedSketch,
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
            selected: SelectedSketch::One,
        }
    }

    fn input(&mut self, event: &WindowEvent, _window: &Window) -> bool {
        if let WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
            ..
        } = event
        {
            println!("Space!! {:?}", self.selected);
            self.selected = if let SelectedSketch::One = self.selected {
                SelectedSketch::Two
            } else {
                SelectedSketch::One
            }
        }
        false
    }

    fn update(&mut self, _window: &Window) {}
}

struct View {
    layer_idx1: usize,
    layer_idx2: usize,
}

impl AppView<State> for View {
    fn init(renderer: &mut Renderer, state: &State) -> Self {
        let form = renderer.make_form_simple_range(3);

        let shade1 = renderer.make_shade(include_str!("shader1.wgsl"));
        let sketch1 = renderer.make_sketch(shade1, form);

        let layer_idx1 = renderer.make_layer(vec![sketch1]);
        renderer
            .layer_mut(layer_idx1)
            .set_clear_color(Some(state.color));

        let shade2 = renderer.make_shade(include_str!("shader2.wgsl"));
        let sketch2 = renderer.make_sketch(shade2, form);

        let layer_idx2 = renderer.make_layer(vec![sketch2]);
        renderer
            .layer_mut(layer_idx2)
            .set_clear_color(Some(state.color));

        Self {
            layer_idx1,
            layer_idx2,
        }
    }

    fn render(&mut self, renderer: &mut Renderer, state: &State) -> Result<(), wgpu::SurfaceError> {
        match state.selected {
            SelectedSketch::One => renderer.render_layer(self.layer_idx1),
            SelectedSketch::Two => renderer.render_layer(self.layer_idx2),
        }
    }

    fn resize(&mut self, _window: &Window) {}
}

fn main() {
    let app = pollster::block_on(App::new(WindowBuilder::new()));
    run::<State, View>(app);
}
