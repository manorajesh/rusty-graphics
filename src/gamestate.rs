use crate::{vector::Vector, raycaster::{MapCellType, MapCell}, DEFAULT_ACCELERATION};

#[derive(Default)]
pub struct GameState {
    pub enemy: Enemy,
    pub player: Player,
    pub map_toggle: bool,
    pub acceleration: f64,
    pub rotation_speed: f64,
}

#[derive(Default)]
pub struct Enemy {
    pub pos: Vector<f64>,
    pub vel: Vector<f64>,
    pub state: EnemyState,
    pub billboard: Billboard,
    pub line_of_sight: bool,
}

#[derive(Default)]
pub struct Billboard(Vec<Vec<[u8; 4]>>);

#[derive(Default)]
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

impl GameState {
    pub fn new() -> Self {
        Self {
            acceleration: DEFAULT_ACCELERATION,
            rotation_speed: 0.001,
            map_toggle: true,
            ..Default::default()
        }
    }
    pub fn update_player(&mut self, map: &Vec<Vec<MapCell>>) {
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
}