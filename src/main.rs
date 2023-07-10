use pixels::Error;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
};

mod raycaster;
mod window;

pub const WIDTH: u32 = 24;
pub const HEIGHT: u32 = 24;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut gw = window::GameWindow::new("2D Raycaster", &event_loop, 1)?;
    let mut raycaster = raycaster::RayCaster::new(60.);
    // gw.pixels.resize_buffer(960, 720).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                // println!("Redraw requested");
                let frame = gw.pixels.frame_mut();

                // Clear the frame
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0, 0, 0, 255]); // Set every pixel to black
                }

                raycaster.draw(frame).unwrap();
                gw.pixels.render().unwrap();
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
                // println!("Window resized to {:?}", size);
                gw.resize((size.width, size.height));
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                // println!("Keyboard input detected");
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Up) if input.state == ElementState::Pressed => {
                        raycaster.change_direction(raycaster::Direction::Up)
                    }
                    Some(VirtualKeyCode::Down) if input.state == ElementState::Pressed => {
                        raycaster.change_direction(raycaster::Direction::Down)
                    }
                    Some(VirtualKeyCode::Left) if input.state == ElementState::Pressed => {
                        raycaster.change_direction(raycaster::Direction::Left)
                    }
                    Some(VirtualKeyCode::Right) if input.state == ElementState::Pressed => {
                        raycaster.change_direction(raycaster::Direction::Right)
                    }
                    _ => {}
                }
            }

            _ => {}
        }

        gw.window.request_redraw();
    });
}

fn verline(frame: &mut [u8], x: usize, y1: usize, y2: usize, rgba: &[u8; 4], thickness: f64) {
    let half_thickness = (thickness / 2.0).ceil() as i64;

    for t in -half_thickness..=half_thickness {
        let x = if ((x as i64 + t) as usize) < WIDTH as usize {
            (x as i64 + t) as usize
        } else {
            x
        };

        for y in y1..=y2 {
            let index = (y * WIDTH as usize + x) * 4;
            if index < frame.len() && index + 3 < frame.len() {
                frame[index] = rgba[0];
                frame[index + 1] = rgba[1];
                frame[index + 2] = rgba[2];
                frame[index + 3] = rgba[3];
            }
        }
    }
}

fn line(frame: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32, color: [u8; 4]) {
    let dx = i32::abs(x2 - x1);
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -i32::abs(y2 - y1);
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x1;
    let mut y = y1;

    loop {
        set_pixel(frame, x, y, color);

        if x == x2 && y == y2 { break }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x += sx;
        }

        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}


pub fn set_pixel(frame: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    let index = (y * WIDTH as i32 + x) as usize * 4;
    frame[index..index+4].copy_from_slice(&color);
}
