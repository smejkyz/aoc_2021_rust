use std::collections::{BTreeSet, HashMap, HashSet};
use bstr::Chars;
use itertools::Itertools;

pub fn p1(raw_input: &str) -> i32{
    let solution: i32 = raw_input
        .lines()
        .map(|line| {
            line
                .split("|")
                .last()
                .unwrap()
                .split(" ")
                .map(|item|has_correct_size(item.len()) as i32)
                .sum::<i32>()
        })
        .sum();
    //println!("solution: {}", solution);
    solution
}


pub fn p2(raw_input: &str) -> i32{
    let map_positions_to_int = create_map();

    let result = raw_input
        .lines()
        .map(|line| p2_per_line(line))
        .sum();
    assert_eq!(result, 1011785);
    result
}

fn p2_per_line(raw_line: &str) -> i32{
    let (first_part, second_part) = raw_line.split_once(" | ").unwrap();
    let result_map = create_result_map(first_part);
    second_part
        .split(" ")
        .map(|coded| decode_string(coded, &result_map))
        .join("")
        .parse::<i32>()
        .unwrap()
}

fn decode_string(input_str: &str, result_map: &Vec<HashSet<char>>) -> i32 {
    if input_str.len() == 2{
        return 1;
    }
    if input_str.len() == 3{
        return 7;
    }
    if input_str.len() == 4 {
        return 4;
    }
    if input_str.len() == 7{
        return 8;
    }
    let mut set: HashSet<char> = HashSet::new();
    for c in input_str.chars() {
        set.insert(c);
    }
    let position = result_map.iter().position(|item| item == &set).unwrap();
    position as i32
}

fn create_result_map(input: &str) -> Vec<HashSet<char>>{
    let mut _input_data = create_input_data(input);
    let (one, seven, four, eight) = find_given_length(&mut _input_data);

    //nine - has len 6 and 4 is subset
    let nine = find_nine(&mut _input_data, &four);
    let zero = find_zero(&mut _input_data, &one);
    let six = find_six(&mut _input_data);

    // remaining 5 and 2 and 3:
    let three = find_three(&mut _input_data, &seven);
    let five = find_five(&mut _input_data, &six);
    let two = _input_data[0].clone();

    vec![zero, one, two, three, four, five, six, seven, eight, nine]
}

fn find_given_length(p0: &mut Vec<HashSet<char>>) -> (HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>){
    let index_one = p0.iter().position(|item| item.len() == 2).unwrap();
    let one = p0[index_one].clone();
    p0.remove(index_one);

    let index_seven = p0.iter().position(|item| item.len() == 3).unwrap();
    let seven = p0[index_seven].clone();
    p0.remove(index_seven);

    let index_four = p0.iter().position(|item| item.len() == 4).unwrap();
    let four = p0[index_four].clone();
    p0.remove(index_four);

    let index_eight = p0.iter().position(|item| item.len() == 7).unwrap();
    let eight = p0[index_eight].clone();
    p0.remove(index_eight);

    (one, seven, four, eight)
}

// fn find_for_given_length(p0: &Vec<HashSet<char>>, given_length: usize) -> HashSet<char>{
//     if let Some(word) = p0.iter().find(| &data| data.len() == given_length){
//         return word.clone()
//     }else{
//         unreachable!("This else branch should never be reached");
//     }
// }

fn find_nine(p0: &mut Vec<HashSet<char>>, four: &HashSet<char>) -> HashSet<char>{
    let index = p0.iter().position(|item| item & &four == *four && item.len() == 6).unwrap();
    let result = p0[index].clone();
    p0.remove(index);
    result
}

fn find_zero(p0: &mut Vec<HashSet<char>>, one: &HashSet<char>) -> HashSet<char>{
    let index = p0.iter().position(|item|  item & &one == *one && item.len() == 6).unwrap();
    let result = p0[index].clone();
    p0.remove(index);
    result
}

fn find_six(p0: &mut Vec<HashSet<char>>) -> HashSet<char>{
    let index = p0.iter().position(|item| item.len() == 6).unwrap();
    let result = p0[index].clone();
    p0.remove(index);
    result
}

fn find_three(p0: &mut Vec<HashSet<char>>, seven: &HashSet<char>) -> HashSet<char>{
    let index = p0.iter().position(|item| item & &seven == *seven).unwrap();
    let result = p0[index].clone();
    p0.remove(index);
    result
}

fn find_five(p0: &mut Vec<HashSet<char>>, six: &HashSet<char>) -> HashSet<char>{
    let index = p0.iter().position(|item| item & &six == *item).unwrap();
    let result = p0[index].clone();
    p0.remove(index);
    result
}

fn create_input_data(input: &str) -> Vec<HashSet<char>>{
    let mut _input_data: Vec<HashSet<char>> = Vec::new();
    for val in input.split(" "){
        let mut set: HashSet<char> = HashSet::new();
        for c in val.chars() {
            set.insert(c);
        }
        _input_data.push(set)
    }
    _input_data
}

fn create_map() -> HashMap<BTreeSet<i32>, i32>{
    let mut map_positions_to_int: HashMap<BTreeSet<i32>, i32> = HashMap::new();

    // Populate the HashMap
    map_positions_to_int.insert([0, 1, 4, 6, 5, 2].iter().cloned().collect(), 0);
    map_positions_to_int.insert([0, 2, 3, 4, 6].iter().cloned().collect(), 2);
    map_positions_to_int.insert([0, 2, 3, 5, 6].iter().cloned().collect(), 3);
    map_positions_to_int.insert([0, 1, 3, 5, 6].iter().cloned().collect(), 5);
    map_positions_to_int.insert([0, 1, 4, 6, 5, 3].iter().cloned().collect(), 6);
    map_positions_to_int.insert([0, 1, 3, 2, 5, 6].iter().cloned().collect(), 9);

    map_positions_to_int
}

fn has_correct_size(value: usize) -> bool{
    let given_lens = vec![2,3,4,7];
    given_lens.contains(&value)
}