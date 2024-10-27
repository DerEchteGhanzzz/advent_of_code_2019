use std::time::Instant;
use parser::get_input;
mod days;

mod parser;

fn main() {
    let start = Instant::now();
    println!("Day 2:");
    println!("Solution A:\n{:?}", days::day_2::solve_a(get_input("2")));
    println!("Solution B:\n{:?}", days::day_2::solve_b(get_input("2")));
    println!("");
    println!("Day 5:");
    println!("Solution A:\n{:?}", days::day_5::solve_a(get_input("5")));
    println!("Solution B:\n{:?}", days::day_5::solve_b(get_input("5")));
    println!("");
    println!("Day 7:");
    println!("Solution A:\n{:?}", days::day_7::solve_a(get_input("7")));
    println!("Solution B:\n{:?}", days::day_7::solve_b(get_input("7")));
    println!("");
    println!("Day 8:");
    println!("Solution A:\n{:?}", days::day_8::solve_a(get_input("8")));
    println!("Solution B:\n{}", days::day_8::solve_b(get_input("8")));
    println!("");
    println!("Day 9:");
    println!("Solution A:\n{:?}", days::day_9::solve_a(get_input("9")));
    println!("Solution B:\n{:?}", days::day_9::solve_b(get_input("9")));
    println!("");
    println!("Day 10:");
    println!("Solution A:\n{:?}", days::day_10::solve_a(get_input("10")));
    println!("Solution B:\n{:?}", days::day_10::solve_b(get_input("10")));
    println!("");
    println!("Day 11:");
    println!("Solution A:\n{:?}", days::day_11::solve_a(get_input("11")));
    println!("Solution B:\n{}", days::day_11::solve_b(get_input("11")));
    println!("");
    println!("Day 12:");
    println!("Solution A:\n{:?}", days::day_12::solve_a(get_input("12")));
    println!("Solution B:\n{}", days::day_12::solve_b(get_input("12")));
    println!("");
    println!("Time elapsed: {:?}", start.elapsed());
}
