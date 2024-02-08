
use clearscreen;
use crate::map::{Map, Tile};
use crate::snake::{Snake, Direction};

mod map;
mod snake;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Game {
    map: Map,
    snake: Snake,
}

impl Game {
    pub fn new(width: usize, height: usize, start_len: usize) -> Self {
        // create snake at the center of the map
        let start_pos = Point{
            x: width / 2,
            y: height / 2
        };
        let snake = Snake::new(start_pos, start_len);

        // initialize a map with border
        let mut map = Map::new(width, height);
        
        for pos in snake.body() {
            map.set_tile(*pos, Tile::Snake);
        }
        
        map.create_border();
        map.spawn_apple();

        Self {map, snake}
    }

    pub fn update(&mut self, input: Option<Direction>) -> Result<(), String> {
        // change the direction of the snake,
        // unless it requires turning 180 degrees
        if let Some(new_direction) = input {
            self.snake.try_change_direction(new_direction);
        }

        // remember the previous tail position
        // so that we can erase outdated trail
        let prev_tail = self.snake.tail();

        // move the snake by one step
        self.snake.move_forward();
        
        // handle collision
        let curr_head = self.snake.head();
        match self.map.tile(curr_head) {
            Tile::Wall => return Err(String::from("collision with wall!")),
            Tile::Snake => return Err(String::from("collision with tail!")),
            Tile::Apple => self.on_eat_apple(),
            Tile::Empty => ()
        }

        // update the tile where the snake's head and tail is on.
        // the if statement is required to prevent deleting
        // newly grown tail of the snake right after eating an apple.
        self.map.set_tile(curr_head, Tile::Snake);
        if self.snake.tail() != prev_tail {
            self.map.set_tile(prev_tail, Tile::Empty);
        }

        Ok(())
    }

    fn on_eat_apple(&mut self) {
        self.snake.append_body();
        self.map.spawn_apple();
    }

    pub fn render(&self) {
        clearscreen::clear().expect("clearscreen should succeed");
        
        for y in 0..self.map.height() {
            for x in 0..self.map.width() {
                print!("{}", tile_to_char(self.map.tile(Point{x, y})));
            }
            println!();
        }
    }
}

fn tile_to_char(tile: &Tile) -> char{
    match tile {
        Tile::Empty => ' ',
        Tile::Wall => '#',
        Tile::Snake => '@',
        Tile::Apple => '$',
    }
}

pub fn direction_from_key(key: char) -> Option<Direction> {
    match key {
        'w' => Some(Direction::Up),
        's' => Some(Direction::Down),
        'd' => Some(Direction::Right),
        'a' => Some(Direction::Left),
        _ => None
    }
}
