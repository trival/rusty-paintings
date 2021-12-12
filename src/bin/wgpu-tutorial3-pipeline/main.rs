use paintings::prelude::*;
use winit::{
    event::*,
    window::{Window, WindowBuilder},
};

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
    sketches2: Vec<Sketch>,
    sketches1: Vec<Sketch>,
    layer: Layer,
}

impl AppView<State> for View {
    fn init(state: &State, renderer: &Renderer) -> Self {
        let form = Form::new(3);
        Self {
            layer: Layer::new().with_clear_color(Some(state.color)),
            sketches1: vec![Sketch::new(
                renderer,
                &Shade::new(renderer, include_str!("shader1.wgsl")),
                &form,
            )],
            sketches2: vec![Sketch::new(
                renderer,
                &Shade::new(renderer, include_str!("shader2.wgsl")),
                &form,
            )],
        }
    }

    fn render(
        &mut self,
        renderer: &paintings::renderer::Renderer,
        state: &State,
    ) -> Result<(), wgpu::SurfaceError> {
        self.layer.render(
            renderer,
            match state.selected {
                SelectedSketch::One => &self.sketches1,
                SelectedSketch::Two => &self.sketches2,
            },
        )
    }

    fn resize(&mut self, _window: &Window) {}
}

fn main() {
    let app = pollster::block_on(App::new(WindowBuilder::new()));
    run::<State, View>(app);
}
