
pub fn p1(raw_input: &str) -> i32 {
    let input = parse_input(raw_input);
    let p1 = compute(input);
    // println!("p1 {}", p1);
    return p1
}


pub fn p2(raw_input: &str) -> i32 {
    let input = parse_input(raw_input);
    // println!("len input {}", input.len());
    let sums: Vec<i32> = input
        .windows(3)
        .map(|chunk| chunk.iter().sum())
        .collect();
    let c = compute(sums);
    // println!("p2 {}", c);
    return c;
}

fn compute(input: Vec<i32>) -> i32{
    let mut c = 0;
    for window in input.windows(2){
        if window[0] < window[1]{
            c = c + 1;
        }
    }
    return c;
}


fn parse_input(raw_input: &str) -> Vec<i32> {
    raw_input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
