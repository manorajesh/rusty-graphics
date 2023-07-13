use std::sync::{Arc, Mutex};

use crate::{line, set_pixel, vector::Vector, verline, ACCELERATION, HEIGHT, WIDTH};
use rayon::prelude::*;

pub struct RayCaster {
    player: Player,
    map: Vec<Vec<u8>>,
    fov: f64,
}

struct Ray {
    dir: Vector<f64>,
    hit: bool,
}

struct Player {
    pub pos: Vector<f64>,
    pub dir: Vector<f64>,
    pub vel: Vector<f64>,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Mouse(f64, f64),
}

impl RayCaster {
    pub fn new(fov: f64) -> Self {
        Self {
            player: Player {
                pos: Vector { x: 22.0, y: 12.0 },
                dir: Vector { x: -1.0, y: 0.0 },
                vel: Vector { x: 0., y: 0. },
            },

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
        }
    }

    pub fn draw(&self, frame: &mut [u8], map_toggle: bool) -> Result<(), String> {
        if map_toggle {
            // map
            for y in 0..self.map.len() {
                for x in 0..self.map[y].len() {
                    let color = match self.map[y][x] {
                        1 => Some([255, 0, 0, 255]),
                        2 => Some([0, 255, 0, 255]),
                        3 => Some([0, 0, 255, 255]),
                        4 => Some([255, 255, 255, 255]),
                        5 => Some([255, 255, 0, 255]),
                        _ => None,
                    };

                    if let Some(color) = color {
                        set_pixel(frame, x, y, color, 1);
                    }
                    // filled_rectangle(frame, x, y, x+1, y+2, color, PIXELSIZE)
                }
            }

            set_pixel(
                frame,
                self.player.pos.x as usize,
                self.player.pos.y as usize,
                [25, 0, 255, 255],
                1,
            );
            line(
                frame,
                self.player.pos.x as isize,
                self.player.pos.y as isize,
                (self.player.pos.x + self.player.dir.x * 10.) as isize,
                (self.player.pos.y + self.player.dir.y * 10.) as isize,
                [255, 0, 0, 255],
                1,
            );

            // orthogonal line
            line(
                frame,
                self.player.pos.x as isize,
                self.player.pos.y as isize,
                (self.player.pos.x + self.player.dir.orthogonal(Direction::Left).x * 10.) as isize,
                (self.player.pos.y + self.player.dir.orthogonal(Direction::Left).y * 10.) as isize,
                [0, 255, 0, 255],
                1,
            );
            line(
                frame,
                self.player.pos.x as isize,
                self.player.pos.y as isize,
                (self.player.pos.x + self.player.dir.orthogonal(Direction::Right).x * 10.) as isize,
                (self.player.pos.y + self.player.dir.orthogonal(Direction::Right).y * 10.) as isize,
                [0, 0, 255, 255],
                1,
            );
            return Ok(());
        }

        // raycasting
        let frame = Arc::new(Mutex::new(frame));
        let half_fov: f64 = self.fov / 2.;
        const NUMRAYS: f64 = WIDTH as f64;
        (0..NUMRAYS as usize).into_par_iter().for_each(|i| {
            let angle = (self.fov / NUMRAYS * i as f64 - half_fov) * 1f64.to_radians();
            let mut ray = Ray {
                dir: self.player.dir.rotate(angle),
                hit: false,
            };

            // map_pos is the current map cell we are in
            let mut map_pos: Vector<i32> = Vector::new(
                self.player.pos.x.floor() as i32,
                self.player.pos.y.floor() as i32,
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
                    (self.player.pos.x - map_pos.x as f64) * delta_dist.x
                } else {
                    // top right edge of map cell
                    (map_pos.x as f64 + 1. - self.player.pos.x) * delta_dist.x
                },

                y: if ray.dir.y < 0. {
                    // top left edge of map cell
                    (self.player.pos.y - map_pos.y as f64) * delta_dist.y
                } else {
                    // top right edge of map cell
                    (map_pos.y as f64 + 1. - self.player.pos.y) * delta_dist.y
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

                if self.map[map_pos.y as usize][map_pos.x as usize] > 0 {
                    ray.hit = true;
                }
            }

            let mut color = match self.map[map_pos.y as usize][map_pos.x as usize] {
                1 => [255, 0, 0, 255],
                2 => [0, 255, 0, 255],
                3 => [0, 0, 255, 255],
                4 => [255, 255, 0, 255],
                5 => [255, 0, 255, 255],
                _ => [255, 255, 255, 255],
            };

            if side == 1 {
                color.div_assign(2)
            }

            let distance: f64 = if side == 0 {
                (map_pos.x as f64 - self.player.pos.x + (1. - step.x) / 2.) / ray.dir.x
            } else {
                (map_pos.y as f64 - self.player.pos.y + (1. - step.y) / 2.) / ray.dir.y
            };

            let correct_distance = distance * (self.player.dir.angle() - ray.dir.angle()).cos();

            let mut height = (HEIGHT as f64 / correct_distance).abs() * 15.;
            if height > HEIGHT as f64 {
                height = HEIGHT as f64;
            }

            let column_start = HEIGHT as usize / 2 - height as usize / 2;
            let column_end = HEIGHT as usize / 2 + height as usize / 2;
            verline(&mut frame.lock().unwrap(), i, column_start, column_end, color, 1);
        });
        Ok(())
    }

    pub fn update_player(&mut self) {
        let new_pos_x = Vector::new(self.player.pos.x + self.player.vel.x, self.player.pos.y);
        if self.is_valid_position(&new_pos_x) {
            self.player.pos = new_pos_x;
        }

        let new_pos_y = Vector::new(self.player.pos.x, self.player.pos.y + self.player.vel.y);
        if self.is_valid_position(&new_pos_y) {
            self.player.pos = new_pos_y;
        }

        self.player.vel *= 0.9;
    }

    fn is_valid_position(&self, pos: &Vector<f64>) -> bool {
        if let Some(row) = self.map.get(pos.y as usize) {
            if let Some(cell) = row.get(pos.x as usize) {
                if *cell == 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn change_direction(&mut self, dir: Direction) {
        const ROTATESPEED: f64 = 0.001;
        let acceleration = unsafe { ACCELERATION };

        match dir {
            Direction::Down => {
                self.player.vel.x -= self.player.dir.x * acceleration;
                self.player.vel.y -= self.player.dir.y * acceleration;
            }
            Direction::Up => {
                self.player.vel.x += self.player.dir.x * acceleration;
                self.player.vel.y += self.player.dir.y * acceleration;
            }
            Direction::Left => {
                let ortho = self.player.dir.orthogonal(Direction::Left);
                self.player.vel.x -= ortho.x * acceleration;
                self.player.vel.y -= ortho.y * acceleration;
            }
            Direction::Right => {
                let ortho = self.player.dir.orthogonal(Direction::Right);
                self.player.vel.x -= ortho.x * acceleration;
                self.player.vel.y -= ortho.y * acceleration;
            }
            Direction::Mouse(dx, _) => {
                self.player.dir = self.player.dir.rotate(dx * ROTATESPEED);
            }
        }
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

fn generate_map() -> Vec<Vec<u8>> {
    let mut map = vec![vec![0u8; 1000]; 1000];

    // Fill the border with 1s
    for i in 0..1000 {
        map[0][i] = 1;
        map[999][i] = 1;
    }
    for i in 0..1000 {
        map[i][0] = 1;
        map[i][999] = 1;
    }

    // Add a large room of 2s at the center
    for i in 250..500 {
        for j in 375..625 {
            map[i][j] = 2;
        }
    }

    // Add a corridor of 3s leading from the room to the right wall
    for i in 375..416 {
        for j in 625..1000 {
            map[i][j] = 3;
        }
    }

    // Add a small room of 4s at the top left
    for i in 62..156 {
        for j in 62..156 {
            map[i][j] = 4;
        }
    }

    // Add a corridor of 5s leading from the small room to the large room
    for i in 140..250 {
        for j in 62..94 {
            map[i][j] = 5;
        }
    }

    // Add a small room of 4s at the bottom right
    for i in 593..688 {
        for j in 844..938 {
            map[i][j] = 4;
        }
    }

    // Add a corridor of 5s leading from the small room to the bottom border
    for i in 688..750 {
        for j in 844..875 {
            map[i][j] = 5;
        }
    }

    // Add a new room of 6s at the top right
    for i in 62..156 {
        for j in 844..938 {
            map[i][j] = 6;
        }
    }

    // Add a new corridor of 7s leading from the new room to the top border
    for i in 0..62 {
        for j in 844..875 {
            map[i][j] = 7;
        }
    }

    map
}
