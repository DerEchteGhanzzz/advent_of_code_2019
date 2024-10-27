use std::collections::HashMap;

#[derive(Debug)]
pub struct IntComputer {
    ptr: usize,
    relative_base: i64,
    memory: HashMap<usize, i64>,
    state: State,
    input: Vec<i64>,
    output: Vec<i64>,
    debug_mode: bool,
    computer_mode: Mode,
}

impl IntComputer {
    pub fn new(program: &str) -> Self {
        let mut memory = HashMap::new();
        program.split(',').map(|i| i.parse::<i64>().unwrap()).enumerate().for_each(|(idx, i)| {memory.insert(idx, i);} );
        IntComputer { ptr: 0, relative_base: 0, memory: memory, state: State::Waiting, input: Vec::new(), output: Vec::new(), debug_mode: false, computer_mode: Mode::Normal }
    }

    pub fn run_code(&mut self) -> () {
        self.state = State::Running;
        while self.state.is_running() {
            let op_code = self.get_address(self.ptr);
            
            if self.debug_mode {
                println!("ptr: {}; op code: {}; value: {:?}; relative base: {:?}", self.ptr, op_code, self.input, self.relative_base)
            }

            match op_code % 100 {
                // 1: add, 2: mult
                o @ (1 | 2) => {
                    let param_addrs = Self::get_param_addresses(&self, op_code, 3);
                    
                    if self.debug_mode {
                        println!("params: {:?}", param_addrs)
                    }
                    let operation = if o == 1 {|a, b| a + b} else {|a, b| a * b};
                    self.set_address(
                        param_addrs[2], 
                        operation(self.get_address(param_addrs[0]), self.get_address(param_addrs[1]))
                    );
                    self.ptr += param_addrs.len() + 1;
                },
                // take from input
                3 => {
                    let params = Self::get_param_addresses(&self, op_code, 1);
                    if self.debug_mode {
                        println!("param addresses: {:?}", params)
                    }

                    let input = self.input.remove(0);
                    self.set_address(
                        params[0], 
                        input
                    );
                    self.ptr += params.len() + 1;
                },
                // write to output
                4 => {
                    let param_addrs = Self::get_param_addresses(&self, op_code, 1);
                    if self.debug_mode {
                        println!("param addresses: {:?}", param_addrs)
                    }
                    self.output.push(self.get_address(param_addrs[0]));
                    self.ptr += param_addrs.len() + 1;
                    match self.computer_mode {
                        Mode::Amp => self.state = State::Waiting,
                        Mode::PaintingRobot => if self.output.len() == 2 { self.state = State::Waiting },
                        _ => {}
                    }
                },
                // 5: jump if not false, 6: jump if false
                o@ (5 | 6) => {
                    let param_addrs = Self::get_param_addresses(&self, op_code, 2);
                    if self.debug_mode {
                        println!("param addresses: {:?}", param_addrs);
                        println!("ptr before: {:?}", self.ptr);
                    }
                    let cmp = if o == 5 { |p: i64| p != 0 } else { |p: i64| p == 0 } ;
                    if cmp(self.get_address(param_addrs[0])) {
                        self.ptr = self.get_address(param_addrs[1]) as usize;
                    } else {
                        self.ptr += param_addrs.len() + 1;
                    }
                    if self.debug_mode {
                        println!("ptr after: {:?}", self.ptr);
                    }
                },
                // 7: jump if smaller, 8: jump if equals
                o @ (7 | 8) => {
                    let param_addrs = Self::get_param_addresses(&self, op_code, 3);

                    if self.debug_mode {
                        println!("param addresses: {:?}", param_addrs)
                    }
                    let cmp: fn(i64, i64) -> bool = if o == 7 { |a: i64, b: i64| a < b } else { |a: i64, b: i64| a == b } ;
                    self.set_address(
                        param_addrs[2], 
                        if cmp(self.get_address(param_addrs[0]), self.get_address(param_addrs[1])) { 1 } else { 0 }
                    );
                    self.ptr += param_addrs.len() + 1;
                },
                // adjust relative base
                9 => {
                    let param_addrs = Self::get_param_addresses(&self, op_code, 1);
                    if self.debug_mode {
                        println!("params: {:?}", param_addrs)
                    }
                    self.relative_base += self.get_address(param_addrs[0]);

                    if self.debug_mode {
                        println!("changing relative base to: {}", self.relative_base)
                    }

                    self.ptr += param_addrs.len() + 1;
                }
                99 => { 
                    self.state = State::Finished 
                },
                n => { let e = format!("unexpected op code {} at index {}", n, self.ptr); println!("{}", e); self.state = State::Error(e); },
            }
        }
    }

    pub fn get_zero_idx(&self) -> Result<i64, String> {
        match &self.state {
            State::Finished => Ok(self.get_address(0)),
            State::Error(e) => Err(e.to_string()),
            State::Waiting => Err(String::from("program has not run yet!")),
            State::Running => Err(String::from("program still running!")),
        }
    }

    pub fn set_n_and_v(&mut self, noun: i64, verb: i64) {
        self.set_address(1, noun);
        self.set_address(2, verb);
    }

    pub fn push_input(&mut self, value: i64) {
        self.input.push(value);
    }

    pub fn get_last_output(&self) -> i64 {
        *self.output.last().unwrap()
    }

    pub fn pop_last_output(&mut self) -> i64 {
        self.output.pop().unwrap()
    }

    pub fn is_finished(&self) -> bool {
        self.state == State::Finished
    }

    pub fn set_debug_mode(&mut self, dm: bool) {
        self.debug_mode = dm;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.computer_mode = mode;
    }

    fn get_param_addresses(&self, op_code: i64, param_amt: usize) -> Vec<usize> {
        Self::to_modes(op_code, param_amt).iter().enumerate().map(|(idx, m)| self.pm_to_address( m, idx)).collect::<Vec<_>>()
    }

    fn to_modes(op_code: i64, param_amt: usize) -> Vec<ParamMode> {
        let fmtted = &format!("{:0>5}", op_code).chars().rev().collect::<Vec<_>>()[2..2+param_amt];
        fmtted.iter().map(|ch| ParamMode::new(ch.to_digit(10).unwrap())).collect()
    }

    fn get_address(&self, address: usize) -> i64 {
        match self.memory.get(&address) {
            None => 0,
            Some(val) => *val,
        }
    }

    fn pm_to_address(&self, mode: &ParamMode, idx: usize) -> usize {
        match mode {
            ParamMode::Position => self.get_address(self.ptr+idx+1) as usize,
            ParamMode::Immediate => self.ptr+idx+1,
            ParamMode::Relative => (self.get_address(self.ptr+idx+1) + self.relative_base) as usize
        }
    }

    fn set_address(&mut self, address: usize, value: i64) {
        self.memory.insert(address, value);
    }

    pub fn get_output(&self) -> Vec<i64> {
        self.output.clone()
    }

    pub fn clear_output(&mut self) {
        self.output = Vec::new();
    }
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Waiting,
    Error(String),
    Running,
    Finished
}

impl State {
    pub fn is_running(&self) -> bool {
        match self {
            Self::Running => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    pub fn new(i: u32) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("cannot parse {i} to param mode")
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    Amp,
    PaintingRobot,
}