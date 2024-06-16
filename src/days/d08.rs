use std::collections::{BTreeSet, HashMap, HashSet};
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

pub fn dep_p1(raw_input: &str) -> i32{
        let mut solution = 0;
        for line in raw_input.lines(){
            //let x: Vec<bool> = line.split("|").last().unwrap().split(" ").map(|item|has_correct_size(item.len())).collect();
            let x: i32 = line.split("|").last().unwrap().split(" ").map(|item|has_correct_size(item.len()) as i32).sum();
            solution += x;
        //println!("tmp");
    }
    println!("solution: {}", solution);
    solution
}

pub fn p2(raw_input: &str) -> i32{
    let map_positions_to_int = create_map();
    let mut result = 0;
    for line in raw_input.lines(){
        let split_vector: Vec<&str> = line.split(" | ").collect();
        let first_part = split_vector[0];
        let second_part =split_vector[1];
        let result_map = create_result_map(first_part);
        let mut per_line_result: Vec<i32> = Vec::new();
        for coded in second_part.split(" "){
            let decoded = decode_string(coded, result_map.clone(), map_positions_to_int.clone());
            per_line_result.push(decoded)
        }
        let line_result = per_line_result.iter().join("").parse::<i32>().unwrap();
        result += line_result;
    }
    //println!("in works, result {}", result);
    result
}

fn decode_string(input_str: &str, result_map: HashMap<char, i32>, map_positions_to_int: HashMap<BTreeSet<i32>, i32>) -> i32 {
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
    let mut int_representation: BTreeSet<i32> = BTreeSet::new();
    for c in input_str.chars() {
        let int_value = result_map[&c];
        int_representation.insert(int_value);
    }
    map_positions_to_int[&int_representation]
}

fn create_result_map(input: &str) -> HashMap<char, i32>{

    let _input_data = create_input_data(input);
    let one = find_for_given_length(&_input_data, 2);
    let seven = find_for_given_length(&_input_data, 3);
    let four = find_for_given_length(&_input_data, 4);
    let zero_or_six_or_nine = find_zero_or_six_or_nine(&_input_data);

    // main algorithm
    let top_position: HashSet<_> = seven.difference(&one).clone().copied().collect();
    assert_eq!(top_position.len(), 1);

    let mut position_top_right: HashSet<_> = seven.intersection(&one).clone().copied().collect();
    let mut position_bottom_right: HashSet<_> = seven.intersection(&one).clone().copied().collect();
    // 3. set possible positions on left - that are in four and not in seven
    let mut position_top_left: HashSet<_> = four.difference(&seven).clone().copied().collect();
    let mut position_middle: HashSet<_> = four.difference(&seven).clone().copied().collect();

    // 4. look at the common at zero_six_nine
    let help: HashSet<_> = zero_or_six_or_nine[0].intersection(&zero_or_six_or_nine[1]).clone().copied().collect();
    let mut common_len_6: HashSet<_> = help.intersection(&zero_or_six_or_nine[2]).clone().copied().collect();
    // # 5. discard the top
    common_len_6.remove(&top_position.iter().collect::<Vec<_>>()[0]);

    // 6. now we can set positions 1 and 3. We should have two choices - one choice is in the common_len_6, the other is not
    // since 0 does not have middle position 3
    position_top_left = common_len_6.intersection(&position_top_left).clone().copied().collect();
    assert_eq!(position_top_left.len(), 1);
    // # 7. update the result_map:
    position_middle = position_middle.difference(&position_top_left).clone().copied().collect();

    // 8. update common:
    common_len_6.remove(&position_top_left.iter().collect::<Vec<_>>()[0]);
    assert_eq!(common_len_6.len(), 2);

    // 9. common_len_5 has 2 elements one has intersection with 7 and does not
    // the intersection has to be position right_bottom
    position_bottom_right = position_bottom_right.intersection(&common_len_6).clone().copied().collect();
    assert_eq!(position_bottom_right.len(), 1);
    // # update result_map
    position_top_right = position_top_right.difference(&position_bottom_right).clone().copied().collect();
    //
    // # update common
    common_len_6.remove(&position_bottom_right.iter().collect::<Vec<_>>()[0]);
    assert_eq!(common_len_6.len(), 1);

    // last value in common_len_6 has to be the bottom:
    let position_bottom = common_len_6;
    // last unknown position is 4 - the remaining
    let mut union_set: HashSet<char> = HashSet::new();
    union_set.extend(top_position.iter());
    union_set.extend(position_top_left.iter());
    union_set.extend(position_top_right.iter());
    union_set.extend(position_middle.iter());
    union_set.extend(position_bottom_right.iter());
    union_set.extend(position_bottom.iter());
    let all_positions: HashSet<char> = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].iter().cloned().collect();
    let position_bottom_left: HashSet<_>  = all_positions.difference(&union_set).clone().copied().collect();

    let mut result: HashMap<char, i32> = HashMap::new();
    // Populate the HashMap
    result.insert(top_position.iter().clone().copied().collect::<Vec<_>>()[0], 0);
    result.insert(position_top_left.iter().clone().copied().collect::<Vec<_>>()[0], 1);
    result.insert(position_top_right.iter().clone().copied().collect::<Vec<_>>()[0], 2);
    result.insert(position_middle.iter().clone().copied().collect::<Vec<_>>()[0], 3);
    result.insert(position_bottom_left.iter().clone().copied().collect::<Vec<_>>()[0], 4);
    result.insert(position_bottom_right.iter().clone().copied().collect::<Vec<_>>()[0], 5);
    result.insert(position_bottom.iter().clone().copied().collect::<Vec<_>>()[0], 6);
    result
}

fn find_zero_or_six_or_nine(p0: &Vec<HashSet<char>>) -> Vec<HashSet<char>> {
    let mut result: Vec<HashSet<char>> = Vec::new();
    for element in p0{
        if element.len() == 6{
            result.push(element.clone());
        }
    }
    result

}

fn find_for_given_length(p0: &Vec<HashSet<char>>, given_length: usize) -> HashSet<char>{
    if let Some(word) = p0.iter().find(| &data| data.len() == given_length){
        return word.clone()
    }else{
        unreachable!("This else branch should never be reached");
    }
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