use crate::days::int_computer;
use itertools::Itertools;

pub fn solve_a(input: Vec<String>) -> i64 {
    let mut max_phase = 0;
    for phase_setting in (0..=4).permutations(5) {
        
        let mut current_phase = 0;
        for amp in 0..5 {
            let mut int_computer = int_computer::IntComputer::new(&input[0]);
            
            int_computer.push_input(phase_setting[amp]);
            int_computer.push_input(current_phase);
            
            int_computer.run_code();
            current_phase = int_computer.pop_last_output();
        }

        if current_phase > max_phase {
            max_phase = current_phase;
        }
    }
	max_phase
}

pub fn solve_b(input: Vec<String>) -> i64 {
	let mut max_phase = 0;
    for phase_setting in (5..=9).permutations(5) {
        let mut amps: Vec<int_computer::IntComputer> = vec![
            int_computer::IntComputer::new(&input[0]),
            int_computer::IntComputer::new(&input[0]),
            int_computer::IntComputer::new(&input[0]),
            int_computer::IntComputer::new(&input[0]),
            int_computer::IntComputer::new(&input[0]),
            ];
        amps.iter_mut().enumerate().for_each(|(i, a)| {a.push_input(phase_setting[i]); a.set_mode(int_computer::Mode::Amp);});
        let mut current_phase = 0;
        let mut amp_idx = 0;
        
        loop {
            amps[amp_idx].push_input(current_phase);
            amps[amp_idx].run_code();

            current_phase = amps[amp_idx].get_last_output();
            amp_idx = (amp_idx + 1) % 5;
            
            if amps.iter().all(|a| a.is_finished()) {
                break;
            }
        }

        if current_phase > max_phase {
            max_phase = current_phase;
        }
    }
	max_phase
}