use winit::{
    event::*, event_loop::EventLoop, keyboard::PhysicalKey, window::WindowBuilder
};

use crate::state::State;

pub(crate) fn create_window() -> (winit::window::Window, EventLoop<()>) {
    let event_loop = EventLoop::new().expect("Error constructing event loop.");
    let window = WindowBuilder::new().build(&event_loop).expect("Error constructing window.");
    (window, event_loop)
}

pub(crate) fn start_event_loop(event_loop: EventLoop<()>, window: &winit::window::Window, state: &mut State) {
    let c_window_id = window.id();
    event_loop.run(|event: Event<()>, event_loop| {
        match event {
            Event::WindowEvent { window_id, event } if window_id == c_window_id => match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                },
                WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                    if event.physical_key == PhysicalKey::Code(winit::keyboard::KeyCode::Escape) {
                        event_loop.exit();
                    }
                },
                WindowEvent::Resized(physical_window_size) => {
                    state.resize(physical_window_size);
                },
                WindowEvent::RedrawRequested => {
                    state.update();
                    match state.render() {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => { eprintln!("Out of memory during rendering."); event_loop.exit(); },
                        Err(e) => eprintln!("Error while rendering: {:?}", e),
                    }
                },
                _ => {},
            },
            Event::AboutToWait => {
                state.window().request_redraw();
            },
            _ => {},
        }
    }).expect("Error in event loop run");
}

