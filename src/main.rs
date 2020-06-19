mod problems;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem_num: u32 = args[1].parse().expect("specify a number pls");

    match problem_num {
        1 => println!("{}", problems::p1::solution()),
        2 => println!("{}", problems::p2::solution()),
        3 => println!("{}", problems::p3::solution()),
        n => println!(":( bad value: {}", n),
    }
}
