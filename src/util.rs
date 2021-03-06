pub const BOARD_SIZE: usize = 3;

pub fn set_board_size() -> usize {
    use std::io;
    println!("hello");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
    input.parse::<usize>().unwrap()
}

pub trait Point {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

impl Point for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }
    fn y(&self) -> usize {
        self.1
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
}

impl Point for Coord {
    fn x(&self) -> usize {
        self.0
    }
    fn y(&self) -> usize {
        self.1
    }
}

impl Into<Coord> for (usize, usize) {
    fn into(self) -> Coord {
        Coord::new(self.0, self.1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Marker {
    X,
    O,
    Empty
}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Marker::X => write!(f, "X"),
            Marker::O => write!(f, "O"),
            Marker::Empty => write!(f, " ")
        }
    }
}
