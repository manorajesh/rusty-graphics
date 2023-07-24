use crate::{vector::Vector, raycaster::{MapCellType, MapCell, distance_squared}, DEFAULT_ACCELERATION};

#[derive(Default)]
pub struct GameState {
    pub enemy: Enemy,
    pub player: Player,
    pub map_toggle: bool,
    pub acceleration: f64,
    pub rotation_speed: f64,
}

#[derive(Debug)]
pub struct Enemy {
    pub pos: Vector<f64>,
    pub vel: Vector<f64>,
    pub dir: Vector<f64>,
    pub state: EnemyState,
    pub billboard: Billboard,
    pub line_of_sight: bool,
}

#[derive(Default, Debug)]
pub struct Billboard(Vec<Vec<[u8; 4]>>);

#[derive(Default, Debug)]
pub enum EnemyState {
    Chasing,
    Hiding,
    #[default]
    Stalking,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Mouse(f64, f64),
}

pub struct Player {
    pub pos: Vector<f64>,
    pub dir: Vector<f64>,
    pub vel: Vector<f64>,
    pub pitch: f64, // -1.0 to 1.0
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Vector::new(0.0, 0.0),
            dir: Vector::new(1.0, 0.0),
            vel: Vector::new(0.0, 0.0),
            pitch: 0.5,
        }
    }
}

impl Default for Enemy {
    fn default() -> Self {
        let billboard = load_billboard("assets/concrete_wall.png");
        Self {
            billboard,
            pos: Vector::new(154.6808076366139, 62.11241662731044),
            vel: Vector::new(0.0, 0.0),
            dir: Vector::new(1.0, 0.0),
            state: EnemyState::default(),
            line_of_sight: false,
        }
    }
}

fn load_billboard(path: &str) -> Billboard {
    let img = image::open(path).unwrap();
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();

    let mut billboard = vec![vec![[0; 4]; width as usize]; height as usize];
    for (y, row) in billboard.iter_mut().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let img_pixel = img.get_pixel(x as u32, y as u32);
            *pixel = [img_pixel[0], img_pixel[1], img_pixel[2], img_pixel[3]];
        }
    }
    Billboard(billboard)
}

impl GameState {
    pub fn new() -> Self {
        Self {
            acceleration: DEFAULT_ACCELERATION,
            rotation_speed: 0.001,
            map_toggle: true,
            ..Default::default()
        }
    }
    pub fn update_positions(&mut self, map: &Vec<Vec<MapCell>>) {
        while !self.is_valid_position(&self.player.pos, map) {
            self.player.pos += 1.;
        }

        let new_pos_x = Vector::new(self.player.pos.x + self.player.vel.x, self.player.pos.y);
        if self.is_valid_position(&new_pos_x, map) {
            self.player.pos = new_pos_x;
        }

        let new_pos_y = Vector::new(self.player.pos.x, self.player.pos.y + self.player.vel.y);
        if self.is_valid_position(&new_pos_y, map) {
            self.player.pos = new_pos_y;
        }

        self.player.vel *= 0.8;
    }

    fn is_valid_position(&self, pos: &Vector<f64>, map: &Vec<Vec<MapCell>>) -> bool {
        if pos.x < 0. || pos.y < 0. {
            return false;
        }
        
        if let Some(row) = map.get(pos.y as usize) {
            if let Some(cell) = row.get(pos.x as usize) {
                if cell.solid == MapCellType::Empty {
                    return true;
                }
            }
        }

        false
    }

    pub fn change_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Down => {
                self.player.vel.x -= self.player.dir.x * self.acceleration;
                self.player.vel.y -= self.player.dir.y * self.acceleration;
            }
            Direction::Up => {
                self.player.vel.x += self.player.dir.x * self.acceleration;
                self.player.vel.y += self.player.dir.y * self.acceleration;
            }
            Direction::Left => {
                let ortho = self.player.dir.orthogonal(Direction::Left);
                self.player.vel.x -= ortho.x * self.acceleration;
                self.player.vel.y -= ortho.y * self.acceleration;
            }
            Direction::Right => {
                let ortho = self.player.dir.orthogonal(Direction::Right);
                self.player.vel.x -= ortho.x * self.acceleration;
                self.player.vel.y -= ortho.y * self.acceleration;
            }
            Direction::Mouse(dx, dy) => {
                self.player.dir = self.player.dir.rotate(dx * self.rotation_speed);
                let new_pitch = self.player.pitch - dy * self.rotation_speed;
                if new_pitch > 0. && new_pitch < 1. {
                    self.player.pitch = new_pitch;
                }
            }
        }
    }

    pub fn billboard_intersection(&self, ray: Vector<f64>) -> Option<Vec<[u8; 4]>> {
        let billboard = &self.enemy.billboard.0;
        let _half_width = billboard[0].len() as f64 / 2.0;
    
        let p1 = self.enemy.pos + self.player.dir.orthogonal(Direction::Left) * 10.;
        let p2 = self.enemy.pos + self.player.dir.orthogonal(Direction::Right) * 10.;

        if !is_between(ray.x, p1.x, p2.x) {
            return None;
        }

        if !is_between(ray.y, p1.y, p2.y) {
            return None;
        }
        
        // get texture for billboard
        // get distance between ray point and p1 (or whichever one is less than the other)
        let least = if p1 < p2 { p1 } else { p2 };
        let dist = ((ray.x - least.x).abs() + (ray.y - least.y).abs()).clamp(0., billboard.len() as f64);
        // let dist = distance_squared(ray, least).sqrt().clamp(0., billboard.len() as f64);

        Some(billboard[dist.round() as usize].clone())
    }
}

fn is_between<T: std::cmp::PartialOrd>(n: T, a: T, b: T) -> bool {
    let (start, end) = if a < b { (a, b) } else { (b, a) };
    start <= n && n <= end
}