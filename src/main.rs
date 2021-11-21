use image::GenericImageView;
use paintings::app::{App, Dimensions};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let size = window.inner_size();

    let mut app = pollster::block_on(App::new(
        &window,
        Dimensions {
            width: size.width,
            height: size.height,
        },
    ));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !app.input(event) {
                // UPDATED!
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

                    WindowEvent::Resized(physical_size) => {
                        app.resize(Dimensions::from(*physical_size))
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        app.resize(Dimensions::from(**new_inner_size));
                    }
                    _ => {}
                }
            }
        }

        Event::RedrawRequested(_) => {
            app.update();
            match app.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}
