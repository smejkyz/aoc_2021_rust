use std::collections::HashSet;
use crate::utils::{benchmark_run, print_day, print_header};
use std::fs;
mod days;
mod utils;

macro_rules! benchmark_all {
    ($($day:ident),*) => {{
        print_header();
        $(
        //let input_path = format!("/Users/user/RustroverProjects/aoc2021/src/inputs/{}.in", &stringify!($day).to_string()[1..]);
        let input_path = "/Users/user/RustroverProjects/aoc2021/src/inputs/11.in";
        //println!("{}", input_path);
        let raw_input = fs::read_to_string(input_path).unwrap();
        let p1_duration = benchmark_run(days::$day::p1, &raw_input);
        let p2_duration = benchmark_run(days::$day::p2, &raw_input);

        print_day(stringify!($day).to_string()[1..].parse().unwrap(), p1_duration, p2_duration);
        )*
    }};
}

fn main() {
    //let a = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    benchmark_all!(d11);
}