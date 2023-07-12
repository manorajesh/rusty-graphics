use pixels::Error;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent, DeviceEvent},
    event_loop::EventLoop,
};

mod raycaster;
mod window;

pub const AA_SCALEFACTOR: u32 = 2;
pub const WIDTH: u32 = 320 * AA_SCALEFACTOR;
pub const HEIGHT: u32 = 240 * AA_SCALEFACTOR;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut gw = window::GameWindow::new("2D Raycaster", &event_loop)?;
    let mut raycaster = raycaster::RayCaster::new(60.);
    let mut map_toggle = false;
    // gw.pixels.resize_buffer(960, 720).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                // println!("Redraw requested");
                let frame = &mut gw.buffer;

                // Clear the frame
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[255, 255, 255, 100]); // Set every pixel to black
                }

                raycaster.update_player();
                raycaster.draw(frame, map_toggle).unwrap();
                gw.downsample();
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
                    Some(VirtualKeyCode::W) if input.state == ElementState::Pressed => {
                        raycaster.change_direction(raycaster::Direction::Up)
                    }
                    Some(VirtualKeyCode::S) if input.state == ElementState::Pressed => {
                        raycaster.change_direction(raycaster::Direction::Down)
                    }
                    Some(VirtualKeyCode::A) if input.state == ElementState::Pressed => {
                        raycaster.change_direction(raycaster::Direction::Left)
                    }
                    Some(VirtualKeyCode::D) if input.state == ElementState::Pressed => {
                        raycaster.change_direction(raycaster::Direction::Right)
                    }
                    Some(VirtualKeyCode::M) if input.state == ElementState::Pressed => {
                        map_toggle = !map_toggle;
                    }
                    _ => {}
                }
            }

            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                raycaster.change_direction(raycaster::Direction::Mouse(delta.0 as f64, delta.1 as f64))
            }

            _ => {}
        }

        gw.window.request_redraw();
    });
}

fn verline(frame: &mut [u8], x: usize, y1: usize, y2: usize, rgba: &[u8; 4], scale: usize) {
    for y in (y1 * scale)..=(y2 * scale) {
        set_pixel(frame, x, y, *rgba, scale, None, None);
    }
}

pub fn line(frame: &mut [u8], x1: isize, y1: isize, x2: isize, y2: isize, color: [u8; 4], scale: usize) {
    if x1 == x2 {
        verline(frame, x1 as usize, y1 as usize, y2 as usize, &color, scale);
        return;
    }
    let dx = isize::abs(x2 - x1) * scale as isize;
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -isize::abs(y2 - y1) * scale as isize;
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x1 * scale as isize;
    let mut y = y1 * scale as isize;

    loop {
        set_pixel(frame, x as usize, y as usize, color, scale, None, None);

        if x == x2 * scale as isize && y == y2 * scale as isize { break }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x += sx * scale as isize;
        }

        if e2 <= dx {
            err += dx;
            y += sy * scale as isize;
        }
    }
}

// fn filled_rectangle(frame: &mut [u8], x1: usize, y1: usize, x2: usize, y2: usize, color: [u8; 4], scale: usize) {
//     for x in (x1*scale)..=(x2*scale) {
//         for y in (y1*scale)..=(y2*scale) {
//             if x >= WIDTH as usize || y >= HEIGHT as usize { continue }
//             set_pixel(frame, x, y, color, scale);
//         }
//     }
// }

pub fn set_pixel(frame: &mut [u8], x: usize, y: usize, color: [u8; 4], scale: usize, width: Option<u32>, height: Option<u32>) {
    let width = if let Some(w) = width { w } else { WIDTH };
    let height = if let Some(h) = height { h } else { HEIGHT };
    for i in 0..scale {
        for j in 0..scale {
            let xi = x * scale + i;
            let yj = y * scale + j;
            if xi < width as usize && yj < height as usize {
                let index = ((yj * width as usize + xi) * 4) as usize;
                if index + 4 <= frame.len() {
                    frame[index..index+4].copy_from_slice(&color);
                }
            }
        }
    }
}

pub fn get_pixel(frame: &[u8], x: usize, y: usize) -> Option<[u8; 4]> {
    let index = ((y * WIDTH as usize + x) * 4) as usize;
    if index + 4 <= frame.len() {
        let color: [u8; 4] = frame[index..index+4].try_into().unwrap();
        Some(color)
    } else {
        None
    }
}