use crate::days::int_computer;

pub fn solve_a(input: Vec<String>) -> Result<i64, String> {
	let mut int_computer = int_computer::IntComputer::new(&input[0]);
    int_computer.set_n_and_v(12, 2);
    int_computer.run_code();
    int_computer.get_zero_idx()
}

pub fn solve_b(input: Vec<String>) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut int_computer = int_computer::IntComputer::new(&input[0]);
            int_computer.set_n_and_v(noun, verb);
            int_computer.run_code();
            if int_computer.get_zero_idx().is_ok_and(|x| x == 19690720) {
                return 100*noun + verb
            }
        }
    }
    -1
}