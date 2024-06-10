use std::cmp::min;
use num::abs;

pub fn p1(raw_input: &str) -> i32 {
    let mut input = parse_input(raw_input);
    input.sort();
    let median = compute_median(&input);
    let result_1 = fn_1(median as i32, &input);
    // println!("p1 {}", result_1);
    result_1
}

fn compute_median(array: &Vec<i32>)->f64{
    if (array.len() % 2)==0 {
        let ind_left = array.len()/2-1;
        let ind_right = array.len()/2 ;
        (array[ind_left]+array[ind_right]) as f64 / 2.0

    } else {
        array[(array.len()/2)] as f64
    }
}

pub fn p2(raw_input: &str) -> i32 {
    let input = parse_input(raw_input);
    let mean = compute_mean(&input);
    let result_2 = min(fn_2(mean.floor() as i32, &input), fn_2(mean.ceil() as i32, &input));
    // println!("p2 {}", result_2);
    result_2
}


fn compute_mean(vector: &Vec<i32>) -> f32{
    let sum: i32 = vector.iter().sum();
    sum as f32 / vector.len() as f32
}


fn parse_input(raw_input: &str) -> Vec<i32> {
    raw_input
        .split(",")
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn fn_1(x: i32, input: &Vec<i32>) -> i32{
    let mut y = 0;
    for element in input.iter(){
        y += abs(element - x)
    }
    return y
}

fn fn_2(x: i32, input: &Vec<i32>) -> i32{
    let mut y = 0;
    for element in input.iter(){
        y += abs(element - x) * (abs(element - x) + 1) / 2;
    }
    return y
}