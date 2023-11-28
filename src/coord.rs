use std::fmt;

#[derive(Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord {
            x: x,
            y: y
        }
    }

    pub fn neighbors(&self) -> Vec<Coord> {
        let mut coords = Vec::new();
        coords.push(Coord::new(self.x, self.y + 1));
        coords.push(Coord::new(self.x, self.y - 1));
        coords.push(Coord::new(self.x + 1, self.y));
        coords.push(Coord::new(self.x + 1, self.y + 1));
        coords.push(Coord::new(self.x + 1, self.y - 1));
        coords.push(Coord::new(self.x - 1, self.y));
        coords.push(Coord::new(self.x - 1, self.y + 1));
        coords.push(Coord::new(self.x - 1, self.y - 1));
        coords
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}

impl Copy for Coord {}

impl Clone for Coord {
    fn clone(&self) -> Self {
        Coord::new(self.x, self.y)
    }
}
