use paintings::prelude::*;
use wgpu::vertex_attr_array;
use winit::window::{Window, WindowBuilder};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: Mat4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: Vec3,
    color: Vec3,
}

struct State {
    bg_color: wgpu::Color,
    vertices: Vec<Vertex>,
    indices: Vec<i32>,
}

const COLOR: (f32, f32, f32) = (0.5, 0.0, 0.5);

fn vertex(pos: Vec3) -> Vertex {
    Vertex {
        position: pos,
        color: COLOR.into(),
    }
}

impl AppState for State {
    fn init() -> Self {
        Self {
            bg_color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            vertices: vec![
                vertex(vec3(-0.0868241, 0.49240386, 0.0)),
                vertex(vec3(-0.49513406, 0.06958647, 0.0)),
                vertex(vec3(-0.21918549, -0.44939706, 0.0)),
                vertex(vec3(0.35966998, -0.3473291, 0.0)),
                vertex(vec3(0.44147372, 0.2347359, 0.0)),
            ],
            indices: vec![0, 1, 4, 1, 2, 4, 2, 3, 4],
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
        let shade = renderer.make_shade(include_str!("shader.wgsl"));
        // let form = renderer.make_form_vertices(
        //     &state.vertices,
        //     &vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        // );
        let form = renderer.make_form_indexed_vertices(
            &state.vertices,
            &vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            &state.indices,
        );
        let sketch = renderer.make_sketch(shade, form);
        let layer = renderer.make_layer(vec![sketch]);
        renderer
            .layer_mut(layer)
            .set_clear_color(Some(state.bg_color));

        Self { layer_idx: layer }
    }

    fn render(
        &mut self,
        renderer: &mut Renderer,
        _state: &State,
    ) -> Result<(), wgpu::SurfaceError> {
        renderer.render_layer(self.layer_idx)
    }

    fn resize(&mut self, _window: &Window) {}
}

fn main() {
    let app = pollster::block_on(App::new(WindowBuilder::new()));
    run::<State, View>(app);
}
