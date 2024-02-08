use rand::Rng;
use crate::Point;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Wall,
    Snake,
    Apple,
}

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::Empty; width * height],
        }
    }

    // surround the boundary with wall tiles.
    pub fn create_border(&mut self) {
        for x in 0..self.width {
            self.set_tile(Point{x, y: 0}, Tile::Wall);
            self.set_tile(Point{x, y: self.height - 1}, Tile::Wall);
        }
        
        for y in 0..self.height {
            self.set_tile(Point{x: 0, y}, Tile::Wall);
            self.set_tile(Point{x: self.width - 1, y}, Tile::Wall);
        }
    }

    // try to find an empty tile and place an apple
    pub fn spawn_apple(&mut self) {
        loop {
            let pos = Point{
                x: rand::thread_rng().gen_range(0..self.width),
                y: rand::thread_rng().gen_range(0..self.height),
            };

            if let Tile::Empty = self.tile(pos) {
                self.set_tile(pos, Tile::Apple);
                return;
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tile(&self, coord: Point) -> &Tile {
        &self.tiles[self.coord_to_index(coord)]
    }

    pub fn set_tile(&mut self, coord: Point, tile: Tile) {
        let index = self.coord_to_index(coord);
        self.tiles[index] = tile;
    }

    fn coord_to_index(&self, coord: Point) -> usize {
        self.width * coord.y + coord.x
    }
}