use std::collections::HashMap;

use crate::days::int_computer;

pub fn solve_a(input: Vec<String>) -> usize {
    let mut robot = PaintingRobot::new(&input[0], 0);
    robot.paint();
    robot.painted_set.len()
}

pub fn solve_b(input: Vec<String>) -> String {
    let mut robot = PaintingRobot::new(&input[0], 1);
    robot.paint();
    robot.display_paint()
}

fn get_colour(map: &HashMap<(i32, i32), bool>, coord: (i32, i32)) -> bool {
    match map.get(&coord) {
        None => false,
        Some(b) => *b,
    }
}

struct PaintingRobot {
    direction: Direction,
    computer: int_computer::IntComputer,
    painted_set: HashMap<(i32, i32), i64>,
    location: (i32, i32),
}

impl PaintingRobot {
    pub fn new(program: &str, starting_colour: i64) -> Self {
        let mut ic = int_computer::IntComputer::new(program);
        ic.set_mode(int_computer::Mode::PaintingRobot);
        PaintingRobot { direction: Direction::Up, computer: ic, painted_set: HashMap::from([((0, 0), starting_colour)]), location: (0, 0)}
    }

    pub fn paint(&mut self) {
        loop {
            let standing_colour = match self.painted_set.get(&self.location) { None => 0, Some(c) => *c};
            self.computer.push_input(standing_colour);

            self.computer.run_code();

            if self.computer.is_finished() {
                break;
            }

            let output = self.computer.get_output();
            self.computer.clear_output();
            let colour = output[0];
            let rotation = output[1];
            
            if rotation == 0 {
                self.direction = self.direction.rotate_left()
            } else {
                self.direction = self.direction.rotate_right()
            }

            if colour == 0 {
                self.painted_set.insert(self.location, 0);
            } else {
                self.painted_set.insert(self.location, 1);
            }

            self.move_forwards();
        }
    }

    fn move_forwards(&mut self) {
        let (x, y) = self.location;
        match self.direction {
            Direction::Up => self.location = (x, y - 1),
            Direction::Left => self.location = (x - 1, y),
            Direction::Down => self.location = (x, y + 1),
            Direction::Right => self.location = (x + 1, y),
        }
    }

    pub fn display_paint(&self) -> String {
        let (left, top) = self.painted_set.iter().fold((0, 0), |(x, y), ((i, j), _)| (x.min(*i), y.max(*j)));
        let (right, bot) = self.painted_set.iter().fold((0, 0), |(x, y), ((i, j), _)| (x.max(*i), y.min(*j)));
        let mut display: Vec<Vec<char>> = Vec::new();

        println!("{},{}; {},{}", left, top, right, bot);
        for y in bot..=top {
            let mut line: Vec<char> = Vec::new();
            for x in left..=right {
                match &self.painted_set.get(&(x, y)) {
                    None | Some(0) => line.push('.'),
                    Some(_) => line.push('#'),
                }
            }
            display.push(line);
        }
        display.iter().map(|chs| chs.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {

    pub fn rotate_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }

}