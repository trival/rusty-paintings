use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::renderer::Renderer;

pub trait AppState {
    fn init() -> Self;
    fn input(&mut self, event: &WindowEvent, window: &Window) -> bool;
    fn update(&mut self, window: &Window);
}

pub trait AppView<State: AppState> {
    fn init(renderer: &mut Renderer, state: &State) -> Self;
    fn resize(&mut self, window: &Window);
    fn render(&mut self, renderer: &mut Renderer, state: &State) -> Result<(), wgpu::SurfaceError>;
}

pub struct App {
    pub window: Window,
    pub renderer: Renderer,
    event_loop: EventLoop<()>,
}

impl App {
    pub async fn new(window_builder: WindowBuilder) -> App {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = window_builder.build(&event_loop).unwrap();

        let renderer = Renderer::new(&window).await;

        Self {
            window,
            renderer,
            event_loop,
        }
    }
}

pub fn run<S: AppState + 'static, V: AppView<S> + 'static>(mut app: App) {
    let mut state = S::init();
    let mut view = V::init(&mut app.renderer, &state);
    app.event_loop
        .run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window.id() => {
                if !state.input(event, &app.window) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,

                        WindowEvent::Resized(_physical_size) => app.renderer.resize(&app.window),
                        WindowEvent::ScaleFactorChanged { .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            app.renderer.resize(&app.window);
                        }
                        _ => {}
                    }
                }
            }

            Event::RedrawRequested(_) => {
                state.update(&app.window);
                match view.render(&mut app.renderer, &state) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => app.renderer.resize(&app.window),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                app.window.request_redraw();
            }
            _ => {}
        });
}
