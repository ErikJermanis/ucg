pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player {
    pub old_position: (u16, u16),
    pub position: (u16, u16),
}

impl Player {
    pub fn new(playfield_origin: (u16, u16)) -> Self {
        Player {
            old_position: (playfield_origin.0 + 1, playfield_origin.1 + 1),
            position: (playfield_origin.0 + 1, playfield_origin.1 + 1),
        }
    }

    pub fn move_player(&mut self, direction: Direction) {
        self.old_position = self.position;
        match direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }
}