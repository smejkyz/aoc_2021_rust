pub fn p1(raw_input: &str) -> i32 {
    let mut horizotal = 0;
    let mut depth = 0;
    for line in raw_input.lines(){
        let mut value = 0;
        if let Some(last_split) = line.split(' ').last() {
            if let Ok(parsed_value) = last_split.parse::<i32>() {
                value = parsed_value;
            }
        }
        //println!("{}", value);
        if let Some(first_char) = line.chars().next(){
            if first_char == 'u'{
                depth = depth - value;
            }
            if first_char == 'd'{
                depth = depth + value;
            }
            if first_char == 'f'{
                horizotal = horizotal + value;
            }
        }
    }
    let p1 = horizotal * depth;
    //println!("p1 {}", p1);
    return p1
}

pub fn p2(raw_input: &str) -> i32 {
    let mut horizotal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in raw_input.lines(){
        let mut value = 0;
        if let Some(last_split) = line.split(' ').last() {
            if let Ok(parsed_value) = last_split.parse::<i32>() {
                value = parsed_value;
            }
        }
        //println!("{}", value);
        if let Some(first_char) = line.chars().next(){
            if first_char == 'u'{
                aim = aim - value;
            }
            if first_char == 'd'{
                aim = aim + value;
            }
            if first_char == 'f'{
                horizotal = horizotal + value;
                depth = depth + value * aim;
            }
        }
    }
    let p1 = horizotal * depth;
    // println!("p1 {}", p1);
    return p1

}

fn parse_input(raw_input: &str){
    for line in raw_input.lines(){
        println!("{}", line);
    }
    // raw_input
    //     .lines()
    //     .map(|line| line.split(" "))
    //     .collect()
}