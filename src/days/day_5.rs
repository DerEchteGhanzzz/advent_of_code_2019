use crate::days::int_computer;

pub fn solve_a(input: Vec<String>) -> Vec<i64> {
	let mut int_computer = int_computer::IntComputer::new(&input[0]);
    int_computer.push_input(1);
    int_computer.run_code();
    int_computer.get_output()
}

pub fn solve_b(input: Vec<String>) -> Vec<i64> {
	let mut int_computer = int_computer::IntComputer::new(&input[0]);
    int_computer.push_input(5);
    int_computer.run_code();
    int_computer.get_output()
}