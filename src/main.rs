use winit::event::*;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use futures::executor::block_on;
use crate::state::State;

mod state;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut state = block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == window.id() =>
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } =>
                        match input {
                            KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    _ => {}
                }
            _ => {}
        }
    });
}
