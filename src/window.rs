use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{HEIGHT, WIDTH, AA_SCALEFACTOR, get_pixel, set_pixel};

pub struct GameWindow {
    pub window: Window,
    pub size: (u32, u32),
    pub pixels: Pixels,
    pub buffer: Vec<u8>,
}

impl GameWindow {
    pub fn new(title: &str, event_loop: &EventLoop<()>) -> Result<Self, Error> {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(event_loop)
            .unwrap();

        let buffer = vec![0u8; WIDTH as usize * HEIGHT as usize * 4];

        let texture_size = (WIDTH/AA_SCALEFACTOR, HEIGHT/AA_SCALEFACTOR);  // Replace with the desired texture size
        let surface_texture = SurfaceTexture::new(texture_size.0, texture_size.1, &window);
        let pixels = Pixels::new(texture_size.0, texture_size.1, surface_texture)?;

        Ok(Self {
            window,
            size: (texture_size.0, texture_size.1),
            pixels,
            buffer,
        })
    }

    pub fn downsample(&mut self) {
        let frame = self.pixels.frame_mut();
        for y in 0..(HEIGHT/AA_SCALEFACTOR) {
            for x in 0..(WIDTH/AA_SCALEFACTOR) {
                let mut avg_pixel = [0u32; 4];
                for i in 0..AA_SCALEFACTOR {
                    for j in 0..AA_SCALEFACTOR {
                        if let Some(pixel) = get_pixel(&self.buffer, (x * AA_SCALEFACTOR + j) as usize, (y * AA_SCALEFACTOR + i) as usize) {
                            avg_pixel[0] += pixel[0] as u32;
                            avg_pixel[1] += pixel[1] as u32;
                            avg_pixel[2] += pixel[2] as u32;
                            avg_pixel[3] += pixel[3] as u32;
                        }
                    }
                }
    
                let avg_pixel = [
                    (avg_pixel[0] / (AA_SCALEFACTOR * AA_SCALEFACTOR)) as u8,
                    (avg_pixel[1] / (AA_SCALEFACTOR * AA_SCALEFACTOR)) as u8,
                    (avg_pixel[2] / (AA_SCALEFACTOR * AA_SCALEFACTOR)) as u8,
                    (avg_pixel[3] / (AA_SCALEFACTOR * AA_SCALEFACTOR)) as u8,
                ];
    
                set_pixel(frame, x as usize, y as usize, avg_pixel, 1, Some(WIDTH/AA_SCALEFACTOR), Some(HEIGHT/AA_SCALEFACTOR));
            }
        }
    }
    
    pub fn resize(&mut self, new_size: (u32, u32)) {
        // If you want to resize the texture when the window is resized,
        // you should replace the new_size here with your desired scaling logic.
        self.pixels.resize_surface(new_size.0, new_size.1).unwrap();
        self.size = new_size;
    }
}
