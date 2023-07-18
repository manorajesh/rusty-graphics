use crate::{line, set_pixel, vector::Vector, verline, HEIGHT, WIDTH, gamestate::GameState};
use image::GenericImageView;

pub struct RayCaster {
    pub map: Vec<Vec<MapCell>>,
    pub wall_texture: Vec<Vec<[u8; 4]>>,
    fov: f64,
}

struct Ray {
    dir: Vector<f64>,
    hit: bool,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct MapCell {
    pub color: [u8; 4],
    pub solid: MapCellType,
    pub height: f64,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum MapCellType {
    Empty,
    Wall,
}

impl MapCell {
    pub fn new(color: [u8; 4], solid: MapCellType, height: f64) -> Self {
        Self {
            color,
            solid,
            height,
        }
    }

    pub fn empty() -> Self {
        Self {
            color: [0, 0, 0, 0],
            solid: MapCellType::Empty,
            height: 0.0,
        }
    }
}

impl RayCaster {
    pub fn new(fov: f64) -> Self {
        Self {
            map: generate_map(),

            // map: [
            //     [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
            //     [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
            //     [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            //     [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
            //   ],
            fov,
            wall_texture: load_texture(),
        }
    }

    pub fn draw(&self, frame: &mut [u8], gs: &GameState) -> Result<(), String> {
        if gs.map_toggle {
            // map
            for y in 0..self.map.len() {
                for x in 0..self.map[y].len() {
                    let dist = distance_squared(Vector::new(x as f64, y as f64), gs.player.pos);
                    if dist > self.fov * self.fov {
                        continue;
                    }

                    let cell = self.map[y][x];

                    // cell.color gets darker farther away from the player
                    let color = [
                        cell.color[0],
                        cell.color[1],
                        cell.color[2],
                        (255. * (1.01 - (dist / (self.fov * self.fov)).powf(0.01))) as u8,
                    ];

                    set_pixel(frame, x, y, color, 1);
                    // filled_rectangle(frame, x, y, x+1, y+2, color, PIXELSIZE)
                }
            }

            set_pixel(
                frame,
                gs.player.pos.x as usize,
                gs.player.pos.y as usize,
                [25, 0, 255, 255],
                1,
            );
            line(
                frame,
                gs.player.pos.x as isize,
                gs.player.pos.y as isize,
                (gs.player.pos.x + gs.player.dir.x * 10.) as isize,
                (gs.player.pos.y + gs.player.dir.y * 10.) as isize,
                [255, 0, 0, 255],
                1,
            );

            // // orthogonal line
            // line(
            //     frame,
            //     self.player.pos.x as isize,
            //     self.player.pos.y as isize,
            //     (self.player.pos.x + self.player.dir.orthogonal(Direction::Left).x * 10.) as isize,
            //     (self.player.pos.y + self.player.dir.orthogonal(Direction::Left).y * 10.) as isize,
            //     [0, 255, 0, 255],
            //     1,
            // );
            // line(
            //     frame,
            //     self.player.pos.x as isize,
            //     self.player.pos.y as isize,
            //     (self.player.pos.x + self.player.dir.orthogonal(Direction::Right).x * 10.) as isize,
            //     (self.player.pos.y + self.player.dir.orthogonal(Direction::Right).y * 10.) as isize,
            //     [0, 0, 255, 255],
            //     1,
            // );
            return Ok(());
        }

        // raycasting
        let half_fov: f64 = self.fov / 2.;
        const NUMRAYS: f64 = WIDTH as f64;
        for i in 0..NUMRAYS as usize {
            let angle = (self.fov / NUMRAYS * i as f64 - half_fov) * 1f64.to_radians();
            let mut ray = Ray {
                dir: gs.player.dir.rotate(angle),
                hit: false,
            };

            // map_pos is the current map cell we are in
            let mut map_pos: Vector<i32> = Vector::new(
                gs.player.pos.x.floor() as i32,
                gs.player.pos.y.floor() as i32,
            );

            // delta of ray to next map cell
            let delta_dist = Vector {
                x: (1.0 / ray.dir.x).abs(),
                y: (1.0 / ray.dir.y).abs(),
            };

            // step direction for map_pos
            let step = Vector {
                x: if ray.dir.x < 0. { -1. } else { 1. },
                y: if ray.dir.y < 0. { -1. } else { 1. },
            };

            // ray distance from side of map cell (helps with determining direction to inc)
            let mut side_dist: Vector<f64> = Vector {
                x: if ray.dir.x < 0. {
                    // top left edge of map cell
                    (gs.player.pos.x - map_pos.x as f64) * delta_dist.x
                } else {
                    // top right edge of map cell
                    (map_pos.x as f64 + 1. - gs.player.pos.x) * delta_dist.x
                },

                y: if ray.dir.y < 0. {
                    // top left edge of map cell
                    (gs.player.pos.y - map_pos.y as f64) * delta_dist.y
                } else {
                    // top right edge of map cell
                    (map_pos.y as f64 + 1. - gs.player.pos.y) * delta_dist.y
                },
            };

            // DDA
            let mut side = 0;
            while !ray.hit {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_pos.x += step.x as i32;
                    side = 0;
                } else {
                    side_dist.y += delta_dist.y;
                    map_pos.y += step.y as i32;
                    side = 1;
                }

                if self.map[map_pos.y as usize][map_pos.x as usize].solid != MapCellType::Empty {
                    ray.hit = true;
                }
            }

            let mut cell = self.map[map_pos.y as usize][map_pos.x as usize];

            if side == 1 {
                cell.color.div_assign(2)
            }

            let distance: f64 = if side == 0 {
                (map_pos.x as f64 - gs.player.pos.x + (1. - step.x) / 2.) / ray.dir.x
            } else {
                (map_pos.y as f64 - gs.player.pos.y + (1. - step.y) / 2.) / ray.dir.y
            };

            let correct_distance = distance * (gs.player.dir.angle() - ray.dir.angle()).cos();
            // let correct_distance = distance;

            let mut height = (HEIGHT as f64 / correct_distance).abs() * 15.;
            if height > HEIGHT as f64 {
                height = HEIGHT as f64;
            }

            let shear = (gs.player.pitch * HEIGHT as f64 / 2.0) as usize;

            let column_start = HEIGHT as usize / 2 - height as usize / 2 + shear;
            let column_end = HEIGHT as usize / 2 + height as usize / 2 + shear;

            // Calculate texture coordinates.
            let wall_x = if side == 0 {
                gs.player.pos.y + correct_distance * ray.dir.y
            } else {
                gs.player.pos.x + correct_distance * ray.dir.x
            };

            
            let tex_x = if side == 0 {
                gs.player.pos.y + correct_distance * ray.dir.y
            } else {
                gs.player.pos.x + correct_distance * ray.dir.x
            };

            let tex_x = (tex_x - tex_x.floor()) * 128.;

            let step = 1. * 128. / 128.;
            let mut tex_pos = (column_start as f64 - shear as f64 - HEIGHT as f64 / 2. + height / 2.) * step;
            for y in column_start..column_end {
                let tex_y = (y - column_start) as f64 / height;
            
                // Map the tex_y to the texture height.
                let tex_y = (tex_y * 128.) as usize;
                
                // Sample the color from the texture.
                let pixel = self.wall_texture[tex_y][tex_x as usize];
                
                // Draw the pixel on the screen.
                set_pixel(frame, i, y, pixel, 1);
            }
        }

        Ok(())
    }
}

trait MulAssign {
    fn mul_assign(&mut self, rhs: f64) -> Self;
}

impl MulAssign for [u8; 4] {
    fn mul_assign(&mut self, rhs: f64) -> Self {
        self[0] = (self[0] as f64 * rhs) as u8;
        self[1] = (self[1] as f64 * rhs) as u8;
        self[2] = (self[2] as f64 * rhs) as u8;
        self[3] = (self[3] as f64 * rhs) as u8;
        *self
    }
}

trait DivAssign {
    fn div_assign(&mut self, rhs: u8);
}

impl DivAssign for [u8; 4] {
    fn div_assign(&mut self, rhs: u8) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
        self[3] /= rhs;
    }
}

fn generate_map() -> Vec<Vec<MapCell>> {
    let img = image::open("assets/map.png").unwrap();
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();

    let mut buffer: Vec<Vec<MapCell>> = vec![vec![MapCell::empty(); width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;
            let solid = if pixel == [0, 0, 0, 0] {
                MapCellType::Empty
            } else {
                MapCellType::Wall
            };
            buffer[y as usize][x as usize] = MapCell::new(pixel, solid, 0.);
        }
    }

    buffer
}

fn distance_squared(p1: Vector<f64>, p2: Vector<f64>) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    dx * dx + dy * dy
}

fn load_texture() -> Vec<Vec<[u8; 4]>> {
    let img = image::open("assets/concrete_wall.png").unwrap();
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();

    let mut buffer: Vec<Vec<[u8; 4]>> = vec![vec![[0, 0, 0, 0]; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0;
            buffer[y as usize][x as usize] = [pixel[0], pixel[1], pixel[2], pixel[3]];
        }
    }

    buffer
}