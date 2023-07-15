use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
};
use winit_input_helper::WinitInputHelper;

mod window;

pub const WIDTH: u32 = 3840;
pub const HEIGHT: u32 = 2160;
pub const SCALEFACTOR: u32 = 2;

fn main() -> Result<(), String> {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let mut gw = window::GameWindow::new("3D World", &event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {

            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // println!("Window closed");
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                println!("Window resized to {:?}", size);
                gw.resize((size.width, size.height));
            }

            // Event::DeviceEvent {
            //     event: DeviceEvent::MouseMotion { delta },
            //     ..
            // } => raycaster.change_direction(raycaster::Direction::Mouse(delta.0, delta.1)),

            _ => {}
        }

        // if input.update(&event) {
        //     if input.key_held(VirtualKeyCode::W) {
        //         raycaster.change_direction(raycaster::Direction::Up)
        //     }

        //     if input.key_held(VirtualKeyCode::S) {
        //         raycaster.change_direction(raycaster::Direction::Down)
        //     }

        //     if input.key_held(VirtualKeyCode::A) {
        //         raycaster.change_direction(raycaster::Direction::Left)
        //     }

        //     if input.key_held(VirtualKeyCode::D) {
        //         raycaster.change_direction(raycaster::Direction::Right)
        //     }

        //     if input.key_pressed(VirtualKeyCode::M) {
        //         map_toggle = !map_toggle;
        //     }
        // }

        gw.window.request_redraw();
    });
}
