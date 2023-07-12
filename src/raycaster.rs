use crate::{line, set_pixel, vector::Vector, HEIGHT, WIDTH};

pub const MAPHEIGHT: usize = 240;
pub const MAPWIDTH: usize = 320;

pub struct RayCaster {
    player: Player,
    map: [[u8; MAPWIDTH]; MAPHEIGHT],
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
            return Ok(());
        }

        // raycasting
        let half_fov: f64 = self.fov / 2.;
        const NUMRAYS: f64 = WIDTH as f64;
        for i in 0..NUMRAYS as usize {
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

            let column_start = HEIGHT as isize / 2 - height as isize / 2;
            let column_end = HEIGHT as isize / 2 + height as isize / 2;
            line(
                frame,
                i as isize,
                column_start,
                i as isize,
                column_end,
                color,
                1,
            );
        }
        Ok(())
    }

    pub fn update_player(&mut self) {
        let new_pos = Vector::new(
            self.player.pos.x + self.player.dir.x * self.player.vel.x,
            self.player.pos.y + self.player.dir.y * self.player.vel.y,
        );
        if self.is_valid_position(&new_pos) {
            self.player.pos = new_pos;
        }
        self.player.vel *= 0.9;
    }

    fn is_valid_position(&self, pos: &Vector<f64>) -> bool {
        if pos.x < 0. || pos.x >= WIDTH as f64 - 1. || pos.y < 0. || pos.y >= HEIGHT as f64 - 1. {
            return false;
        }
        if self.map[pos.y as usize][pos.x as usize] != 0 {
            return false;
        }
        true
    }

    pub fn change_direction(&mut self, dir: Direction) {
        const ACCELERATION: f64 = 0.5;
        const ROTATESPEED: f64 = 0.001;

        match dir {
            Direction::Down => {
                self.player.vel -= ACCELERATION;
            }
            Direction::Up => {
                self.player.vel += ACCELERATION;
            }
            Direction::Left => {
                let perp_left = self.player.dir.rotate(90.);
                self.player.vel.x -= perp_left.x * ACCELERATION;
                self.player.vel.y -= perp_left.y * ACCELERATION;
            }
            Direction::Right => {
                let perp_right = self.player.dir.rotate(-90.);
                self.player.vel.x += perp_right.x * ACCELERATION;
                self.player.vel.y += perp_right.y * ACCELERATION;
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

fn generate_map() -> [[u8; 320]; 240] {
    let mut map = [[0u8; 320]; 240];

    // Fill the border with 1s
    for i in 0..320 {
        map[0][i] = 1;
        map[239][i] = 1;
    }
    for i in 0..240 {
        map[i][0] = 1;
        map[i][319] = 1;
    }

    // Add a large room of 2s at the center
    for i in 80..160 {
        for j in 120..200 {
            map[i][j] = 2;
        }
    }

    // Add a corridor of 3s leading from the room to the right wall
    for i in 120..130 {
        for j in 200..320 {
            map[i][j] = 3;
        }
    }

    // Add a small room of 4s at the top left
    for i in 20..50 {
        for j in 20..50 {
            map[i][j] = 4;
        }
    }

    // Add a corridor of 5s leading from the small room to the large room
    for i in 45..80 {
        for j in 20..30 {
            map[i][j] = 5;
        }
    }

    // Add a small room of 4s at the bottom right
    for i in 190..220 {
        for j in 270..300 {
            map[i][j] = 4;
        }
    }

    // Add a corridor of 5s leading from the small room to the bottom border
    for i in 220..240 {
        for j in 270..280 {
            map[i][j] = 5;
        }
    }

    map
}
