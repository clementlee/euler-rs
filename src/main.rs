mod lib;
mod problems;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem_num: u32 = args[1].parse().expect("specify a number pls");

    match problem_num {
        1 => println!("{}", problems::p1::solution()),
        2 => println!("{}", problems::p2::solution()),
        3 => println!("{}", problems::p3::solution()),
        4 => println!("{}", problems::p4::solution()),
        5 => println!("{}", problems::p5::solution()),
        6 => println!("{}", problems::p6::solution()),
        7 => println!("{}", problems::p7::solution()),
        8 => println!("{}", problems::p8::solution()),
        9 => println!("{}", problems::p9::solution()),
        10 => println!("{}", problems::p10::solution()),
        11 => println!("{}", problems::p11::solution()),
        12 => println!("{}", problems::p12::solution()),
        13 => println!("{}", problems::p13::solution()),
        14 => println!("{}", problems::p14::solution()),
        15 => println!("{}", problems::p15::solution()),
        16 => println!("{}", problems::p16::solution()),
        17 => println!("{}", problems::p17::solution()),
        18 => println!("{}", problems::p18::solution()),
        19 => println!("{}", problems::p19::solution()),
        20 => println!("{}", problems::p20::solution()),
        21 => println!("{}", problems::p21::solution()),
        22 => println!("{}", problems::p22::solution()),
        23 => println!("{}", problems::p23::solution()),
        24 => println!("{}", problems::p24::solution()),
        n => println!(":( bad value: {}", n),
    }
}
