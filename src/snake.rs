use crate::Point;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn is_opposite(&self, dir: &Direction) -> bool {
        match self {
            Direction::Up => *dir == Direction::Down,
            Direction::Down => *dir == Direction::Up,
            Direction::Right => *dir == Direction::Left,
            Direction::Left => *dir == Direction::Right,
        }
    }
}

#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    body: Vec<Point>,
}

impl Snake {
    pub fn new(start_pos: Point, start_len: usize) -> Self {
        Self {
            direction: Direction::Right,
            body: vec![start_pos; start_len],
        }
    }

    pub fn move_forward(&mut self) {
        // starting from the tail,
        // make each body tile follow its predecessor.
        for trail_index in (1..self.body.len()).rev() {
            self.body[trail_index] = self.body[trail_index - 1];
        }

        // update the head's position
        match self.direction {
            Direction::Up => self.body[0].y -= 1,
            Direction::Down => self.body[0].y += 1,
            Direction::Right => self.body[0].x += 1,
            Direction::Left => self.body[0].x -= 1,
        };
    }

    // make the body longer by one tile.
    // the newly grown tail is placed on the previous tail's location.
    pub fn append_body(&mut self) {
        self.body.push(self.tail());
    }

    pub fn try_change_direction(&mut self, new_direction: Direction) {
        // prevent turning 180 degrees
        if self.direction.is_opposite(&new_direction) {
            return;
        }
        self.direction = new_direction;
    }

    pub fn head(&self) -> Point {
        self.body.first()
            .cloned()
            .expect("body is always not empty")
    }

    pub fn tail(&self) -> Point {
        self.body.last()
            .cloned()
            .expect("body is always not empty")
    }

    pub fn body(&self) -> &[Point] {
        &self.body
    }
}