use pixels::Error;
use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
};
use winit_input_helper::WinitInputHelper;

mod raycaster;
mod vector;
mod window;

pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;
pub const SCALEFACTOR: u32 = 1;

pub static mut ACCELERATION: f64 = 0.1;

fn main() -> Result<(), Error> {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let mut gw = window::GameWindow::new("Game", &event_loop)?;
    let mut raycaster = raycaster::RayCaster::new(60.);
    let mut map_toggle = false;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                // println!("Redraw requested");
                let frame = gw.pixels.frame_mut();

                // Clear the frame
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0, 0, 0, 0]); // Set every pixel to black
                }

                raycaster.update_player();

                raycaster.draw(frame, map_toggle).unwrap();
                gw.pixels.render().unwrap();
                let elapsed = now.elapsed().as_millis();
                println!("FPS: {}", 1000 / elapsed)
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

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => raycaster.change_direction(raycaster::Direction::Mouse(delta.0, delta.1)),

            _ => {}
        }

        if input.update(&event) {
            if input.held_shift() {
                unsafe {
                    ACCELERATION = 0.5;
                }
            } else {
                unsafe {
                    ACCELERATION = 0.1;
                }
            }

            if input.key_held(VirtualKeyCode::W) {
                raycaster.change_direction(raycaster::Direction::Up)
            }

            if input.key_held(VirtualKeyCode::S) {
                raycaster.change_direction(raycaster::Direction::Down)
            }

            if input.key_held(VirtualKeyCode::A) {
                raycaster.change_direction(raycaster::Direction::Left)
            }

            if input.key_held(VirtualKeyCode::D) {
                raycaster.change_direction(raycaster::Direction::Right)
            }

            if input.key_pressed(VirtualKeyCode::M) {
                map_toggle = !map_toggle;
            }
        }

        gw.window.request_redraw();
    });
}

fn verline(frame: &mut [u8], x: usize, y1: usize, y2: usize, rgba: [u8; 4], scale: usize) {
    for y in (y1 * scale)..=(y2 * scale) {
        set_pixel(frame, x, y, rgba, scale);
    }
}

pub fn line(
    frame: &mut [u8],
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
    color: [u8; 4],
    scale: usize,
) {
    if x1 == x2 {
        verline(frame, x1 as usize, y1 as usize, y2 as usize, color, scale);
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
        set_pixel(frame, x as usize, y as usize, color, scale);

        if x == x2 * scale as isize && y == y2 * scale as isize {
            break;
        }

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

fn filled_rectangle(
    frame: &mut [u8],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: [u8; 4],
    scale: usize,
) {
    for x in (x1 * scale)..=(x2 * scale) {
        for y in (y1 * scale)..=(y2 * scale) {
            if x >= WIDTH as usize || y >= HEIGHT as usize {
                continue;
            }
            set_pixel(frame, x, y, color, scale);
        }
    }
}

pub fn set_pixel(frame: &mut [u8], x: usize, y: usize, color: [u8; 4], scale: usize) {
    for i in 0..scale {
        for j in 0..scale {
            let xi = x * scale + i;
            let yj = y * scale + j;
            if xi < WIDTH as usize && yj < HEIGHT as usize {
                let index = (yj * WIDTH as usize + xi) * 4;
                if index + 4 <= frame.len() {
                    frame[index..index + 4].copy_from_slice(&color);
                }
            }
        }
    }
}
