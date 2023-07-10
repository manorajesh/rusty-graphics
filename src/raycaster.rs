use crate::{verline, HEIGHT, WIDTH, line, set_pixel, filled_rectangle};

pub const MAPHEIGHT: usize = 24;
pub const MAPWIDTH: usize = 24;

pub const PIXELSIZE: usize = 5;

pub struct RayCaster {
    player: Player,
    map: [[usize; MAPWIDTH]; MAPHEIGHT],
    fov: f64,
}

struct Player {
    pub pos: Vector<f64>,
    pub dir: Vector<f64>,
}

#[derive(Clone, Copy, PartialEq)]
struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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
}

impl RayCaster {
    pub fn new(fov: f64) -> Self {
        Self {
            player: Player {
                pos: Vector { x: 22.0, y: 12.0 },
                dir: Vector { x: -1.0, y: 0.0 },
            },

            map: [
                [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
                [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
                [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
                [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
              ],

              fov,
        }
    }

    pub fn draw(&self, frame: &mut [u8]) -> Result<(), String> {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let color = match self.map[y][x] {
                    1 => [255, 0, 0, 255],
                    2 => [0, 255, 0, 255],
                    3 => [0, 0, 255, 255],
                    4 => [255, 255, 255, 255],
                    5 => [255, 255, 0, 255],
                    _ => [0, 0, 0, 255],
                };

                set_pixel(frame, x, y, color, PIXELSIZE);
                // filled_rectangle(frame, x, y, x+1, y+2, color, PIXELSIZE)
            }
        }

        set_pixel(frame, self.player.pos.x as usize, self.player.pos.y as usize, [255, 255, 255, 255], PIXELSIZE);
        Ok(())
    }

    pub fn change_direction(&mut self, dir: Direction) {
        const MOVESPEED: f64 = 1.;
        match dir {
            Direction::Down => {
                const ANGLE: f64 = std::f64::consts::PI * 0.02; // Rotate by approximately 1 degree
                let (sin, cos) = ANGLE.sin_cos();
                let new_dir_x = self.player.dir.x * cos - self.player.dir.y * sin;
                let new_dir_y = self.player.dir.x * sin + self.player.dir.y * cos;
                self.player.dir = Vector::new(new_dir_x, new_dir_y);
            },
            Direction::Up => {
                const ANGLE: f64 = -std::f64::consts::PI * 0.02; // Rotate by approximately -1 degree
                let (sin, cos) = ANGLE.sin_cos();
                let new_dir_x = self.player.dir.x * cos - self.player.dir.y * sin;
                let new_dir_y = self.player.dir.x * sin + self.player.dir.y * cos;
                self.player.dir = Vector::new(new_dir_x, new_dir_y);
            },
            Direction::Left => {
                self.player.pos += self.player.dir * MOVESPEED;
            },
            Direction::Right => {
                self.player.pos -= self.player.dir * MOVESPEED;
            },
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
