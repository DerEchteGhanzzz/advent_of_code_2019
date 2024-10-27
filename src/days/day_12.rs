pub fn solve_a(input: Vec<String>) -> i64 {
    0
}

pub fn solve_b(input: Vec<String>) -> i64 {
    0
}

struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    pub fn new(position: Position, velocity: Velocity) -> Self {
        Moon{position, velocity}
    }

    pub fn attract(&self, other_moon: &Moon) -> Self {
        Moon::new(
            (
                (other_moon.position.0 - self.position.0).signum(), 
                (other_moon.position.1 - self.position.1).signum(), 
                (other_moon.position.2 - self.position.2).signum()), 
                self.velocity
            )
    }
}

type Velocity = (i32, i32, i32);
type Position = (i32, i32, i32);