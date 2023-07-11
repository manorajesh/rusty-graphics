use crate::{verline, HEIGHT, WIDTH, line, set_pixel, filled_rectangle};

pub const MAPHEIGHT: usize = 240;
pub const MAPWIDTH: usize = 320;

pub const PIXELSIZE: usize = 1;

pub struct RayCaster {
    player: Player,
    map: [[u8; MAPWIDTH]; MAPHEIGHT],
    fov: f64,
}

struct Ray {
    dir: Vector<f64>,
    distance: f64,
    hit: bool,
}

struct Player {
    pub pos: Vector<f64>,
    pub dir: Vector<f64>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn rotate(&self, angle: f64) -> Vector<T>
    where
        T: Into<f64> + From<f64> + Copy,
    {
        let x = self.x.into();
        let y = self.y.into();

        let new_x = (x * angle.cos() - y * angle.sin()).into();
        let new_y = (x * angle.sin() + y * angle.cos()).into();

        Vector::new(new_x, new_y)
    }
}

impl<T> std::ops::Add for Vector<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> std::ops::AddAssign for Vector<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> std::ops::MulAssign for Vector<T>
where
    T: std::ops::MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> std::ops::SubAssign for Vector<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> std::ops::Mul<T> for Vector<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
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

    pub fn draw(&self, frame: &mut [u8]) -> Result<(), String> {
        
        // raycasting
        // let half_fov = self.fov as isize / 2;
        const NUMRAYS: f64 = WIDTH as f64;
        for i in 0..NUMRAYS as usize {
            let angle = self.fov/NUMRAYS * i as f64 * 1f64.to_radians();
            let mut ray = Ray {
                dir: self.player.dir.rotate(angle),
                distance: 0.,
                hit: false,
            };

            let mut pos = self.player.pos;
            let mut side = 0;
            while !ray.hit {
                pos.x += ray.dir.x;
                if self.map[pos.y as usize][pos.x as usize] != 0 {
                    ray.hit = true;
                    break;
                }

                ray.distance += 0.5;

                pos.y += ray.dir.y;
                if self.map[pos.y as usize][pos.x as usize] != 0 {
                    ray.hit = true;
                    side = 1;
                }

                ray.distance += 0.5;
            }

            let mut color = match self.map[pos.y as usize][pos.x as usize] {
                1 => [255, 0, 0, 255],
                2 => [0, 255, 0, 255],
                3 => [0, 0, 255, 255],
                4 => [255, 255, 0, 255],
                5 => [255, 0, 255, 255],
                _ => [255, 255, 255, 255],
            };

            if side == 1 {
                color[0] /= 2;
                color[1] /= 2;
                color[2] /= 2;
            }

            // let correct_distance = ray.distance * angle.cos();

            let height = (HEIGHT as f64 / ray.distance) * 10.;

            let column_start = HEIGHT as isize / 2 - height as isize / 2;
            let column_end = HEIGHT as isize / 2 + height as isize / 2;
            line(frame, i as isize, column_start, i as isize, column_end, color, 1);

            // if ray.distance < 5. {
            //     color = [255, 0, 0, 255];
            // } else if ray.distance < 10. {
            //     color = [255, 255, 0, 255];
            // } else if ray.distance < 15. {
            //     color = [0, 255, 0, 255];
            // } else if ray.distance < 20. {
            //     color = [0, 255, 255, 255];
            // } else if ray.distance < 25. {
            //     color = [0, 0, 255, 255];
            // }

            // let height = (1. / ray.distance) * 100.;

            // filled_rectangle(frame, i, 0, i+1, height as usize, color, PIXELSIZE);

            // line(frame, self.player.pos.x as isize, self.player.pos.y as isize, pos.x as isize, pos.y as isize, color, 1);
        }

        // map
        // for y in 0..self.map.len() {
        //     for x in 0..self.map[y].len() {
        //         let color = match self.map[y][x] {
        //             1 => Some([255, 0, 0, 255]),
        //             2 => Some([0, 255, 0, 255]),
        //             3 => Some([0, 0, 255, 255]),
        //             4 => Some([255, 255, 255, 255]),
        //             5 => Some([255, 255, 0, 255]),
        //             _ => None,
        //         };

        //         if let Some(color) = color {
        //             set_pixel(frame, x, y, color, PIXELSIZE);
        //         }
        //         // filled_rectangle(frame, x, y, x+1, y+2, color, PIXELSIZE)
        //     }
        // }

        // set_pixel(frame, self.player.pos.x as usize, self.player.pos.y as usize, [255, 255, 255, 255], 1);
        Ok(())
    }

    pub fn change_direction(&mut self, dir: Direction) {
        const MOVESPEED: f64 = 1.;
        let old_pos = self.player.pos;
        match dir {
            Direction::Down => {
                self.player.pos += Vector::new(0., 1.) * MOVESPEED;
            },
            Direction::Up => {
                self.player.pos += Vector::new(0., -1.) * MOVESPEED;
            },
            Direction::Left => {
                self.player.dir = self.player.dir.rotate(-10f64.to_radians());
            },
            Direction::Right => {
                self.player.dir = self.player.dir.rotate(10f64.to_radians());
            },
            Direction::Mouse(dx, dy) => {
                self.player.pos += Vector::new(dx as f64, dy as f64);

                if self.player.pos.x < 0. {
                    self.player.pos.x = 0.;
                } else if self.player.pos.x > WIDTH as f64-1. {
                    self.player.pos.x = WIDTH as f64-1.;
                }

                if self.player.pos.y < 0. {
                    self.player.pos.y = 0.;
                } else if self.player.pos.y > HEIGHT as f64-1. {
                    self.player.pos.y = HEIGHT as f64-1.;
                }
            }
        }

        if self.map[self.player.pos.y as usize][self.player.pos.x as usize] != 0 {
            self.player.pos = old_pos;
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
