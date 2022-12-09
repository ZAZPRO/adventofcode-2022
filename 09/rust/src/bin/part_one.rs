use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashSet;
use std::error::Error;

// Possible move directions.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Zero,
    Up,
    Down,
    Left,
    Right,
    Diag(Diagonal),
}

// Diaginal angle direction.
#[derive(Debug, Clone, Copy)]
pub enum Diagonal {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

// Struct to hold each move.
#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub dir: Direction,
    pub amount: u32,
}

impl Move {
    pub fn new(dir: Direction, amount: u32) -> Self {
        Self { dir, amount }
    }

    // Create a move from reference point of 0,0.
    pub fn from_point(point: Point) -> Self {
        match (point.x, point.y) {
            (0, 0) => Self {
                dir: Direction::Zero,
                amount: 0,
            },
            (1..=i32::MAX, 0) => Self {
                dir: Direction::Right,
                amount: point.x as u32,
            },
            (i32::MIN..=-1, 0) => Self {
                dir: Direction::Left,
                amount: -point.x as u32,
            },
            (0, 1..=i32::MAX) => Self {
                dir: Direction::Down,
                amount: point.y as u32,
            },
            (0, i32::MIN..=-1) => Self {
                dir: Direction::Up,
                amount: -point.y as u32,
            },
            (i32::MIN..=-1, i32::MIN..=-1) => Self {
                dir: Direction::Diag(Diagonal::TopLeft),
                amount: point.distance(&Point { x: 0, y: 0 }),
            },
            (1..=i32::MAX, i32::MIN..=-1) => Self {
                dir: Direction::Diag(Diagonal::TopRight),
                amount: point.distance(&Point { x: 0, y: 0 }),
            },
            (1..=i32::MAX, 1..=i32::MAX) => Self {
                dir: Direction::Diag(Diagonal::BottomRight),
                amount: point.distance(&Point { x: 0, y: 0 }),
            },
            (i32::MIN..=-1, 1..=i32::MAX) => Self {
                dir: Direction::Diag(Diagonal::BottomLeft),
                amount: point.distance(&Point { x: 0, y: 0 }),
            },
        }
    }
}

// Struct to hold x & y coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Linear distance between 2 points.
    fn distance(&self, other: &Self) -> u32 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt() as u32
    }

    // Difference between 2 points.
    fn diff(&self, other: &Self) -> Point {
        Point::new(other.x - self.x, other.y - self.y)
    }
}

// Head of rope.
#[derive(Debug, Clone, Copy, Default)]
struct Head {
    pos: Point,
}

impl Head {
    // Move functione of head of the rope.
    fn mov(&mut self, dir: Direction) {
        match dir {
            Direction::Zero => (),
            Direction::Up => self.pos.y -= 1,
            Direction::Down => self.pos.y += 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1,
            _ => todo!("Head can't move in other directions!"),
        }
    }
}

// Tail of the rope.
#[derive(Debug, Clone, Copy, Default)]
struct Tail {
    pos: Point,
}

impl Tail {
    // Move function of tail.
    fn mov(&mut self, mov: Move) {
        for _ in 0..mov.amount {
            match mov.dir {
                Direction::Zero => (),
                Direction::Up => self.pos.y -= 1,
                Direction::Down => self.pos.y += 1,
                Direction::Left => self.pos.x -= 1,
                Direction::Right => self.pos.x += 1,
                Direction::Diag(d) => match d {
                    Diagonal::TopLeft => {
                        self.pos.x -= 1;
                        self.pos.y -= 1;
                    }
                    Diagonal::TopRight => {
                        self.pos.x += 1;
                        self.pos.y -= 1;
                    }
                    Diagonal::BottomLeft => {
                        self.pos.x -= 1;
                        self.pos.y += 1;
                    }
                    Diagonal::BottomRight => {
                        self.pos.x += 1;
                        self.pos.y += 1;
                    }
                },
            }
        }
    }

    // Function that calculates how tail should move to head.
    fn move_to_head(&mut self, head: Head) {
        let distance = self.pos.distance(&head.pos);
        if distance > 1 {
            let diff = self.pos.diff(&head.pos);
            let mut mov = Move::from_point(diff);
            mov = Move {
                dir: mov.dir,
                amount: mov.amount - 1,
            };
            self.mov(mov);
        }
    }
}

// Struct to hold rope itself.
#[derive(Debug, Clone, Default)]
struct Rope {
    head: Head,
    tail: Tail,
}

impl Rope {
    // Move whole rope.
    fn mov(&mut self, command: Move) {
        self.head.mov(command.dir);
        self.tail.move_to_head(self.head);
    }
}

// Nom function to parse move instuctions.
fn move_parser(i: &str) -> IResult<&str, Move> {
    let (i, res) = separated_pair(is_a("RLUD"), tag(" "), nom::character::complete::u32)(i)?;
    match res.0 {
        "R" => Ok((i, Move::new(Direction::Right, res.1))),
        "L" => Ok((i, Move::new(Direction::Left, res.1))),
        "U" => Ok((i, Move::new(Direction::Up, res.1))),
        "D" => Ok((i, Move::new(Direction::Down, res.1))),
        _ => todo!("Invalid move input"),
    }
}

// Nom function to parse whole file.
fn file_parser(i: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, move_parser)(i)
}

// Main program entry.
fn main() -> Result<(), Box<dyn Error>> {
    // Read input file into String.
    let file = std::fs::read_to_string("../input.txt")?;
    // Parse file into List of moves.
    let (_, moves) = file_parser(file.as_str()).unwrap();

    // Store each rope just in case. Pushing a copy of rope here will remembember it's positions.
    let mut rope_history: Vec<Rope> = Vec::new();
    // Create rope that will follow move instuctions.
    let mut rope = Rope::default();
    // Push a copy of it to a list of rope history.
    rope_history.push(rope.clone());

    // For each move in parsed moves.
    for mov in moves {
        // Move X amount of times.
        for _ in 0..mov.amount {
            rope.mov(mov);
            // Push a copy of the move result to a history.
            rope_history.push(rope.clone());
        }
    }

    // We will use HashSet to store unique Points which tail has visited.
    let mut positions_history: HashSet<Point> = HashSet::new();
    // For each entry in rope history.
    for history in rope_history {
        // Insert tail position to a position history. Only unique values will be stored.
        positions_history.insert(history.tail.pos);
    }

    // Cound amount of entries stored in the HashSet. This is our puzzle result.
    println!("Result: {}", positions_history.len());

    Ok(())
}
