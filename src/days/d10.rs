use std::collections::HashMap;
use itertools::Itertools;

pub fn p1(raw_input: &str) -> i32{
    let mut points_table = HashMap::new();
    points_table.insert(')', 3);
    points_table.insert(']', 57);
    points_table.insert('}', 1197);
    points_table.insert('>', 25137);

    let mut close_to_open_states = HashMap::new();
    close_to_open_states.insert(')', '(');
    close_to_open_states.insert(']', '[');
    close_to_open_states.insert('}', '{');
    close_to_open_states.insert('>', '<');

    let score = raw_input
        .lines()
        .map(|line| corrupted_score(line, &points_table, &close_to_open_states))
        .sum();

    //println!("score: {}", score);
    score
}

fn corrupted_score(raw_line: &str, points_table: &HashMap<char, i32>, close_to_open_states: &HashMap<char, char>) -> i32{
    let mut open_states_queue = Vec::new();
    let open_values: Vec<char> = close_to_open_states.values().cloned().collect();
    for char in raw_line.chars(){
        if open_values.contains(&char){
            // opening new state
            open_states_queue.push(char)
        }else {
            // it has to be closing state
            let coresponding_open_state = close_to_open_states.get(&char).unwrap();
            let last_value_in_queue = open_states_queue[open_states_queue.len() - 1];
            if coresponding_open_state == &last_value_in_queue{
                // its ok closing the corresponding open values
                // remove last element from open_states_queue
                open_states_queue.remove(open_states_queue.len() - 1);
            }else {
                // problem - closing something that should not be closed!
                return points_table.get(&char).unwrap().clone() // todo: is clone necessary?
            }

        }

    }
    //Line is fine :tada. Corrupted score is 0
    0
}


pub fn p2(raw_input: &str) -> i64{
    let mut points_table = HashMap::new();
    points_table.insert(')', 1);
    points_table.insert(']', 2);
    points_table.insert('}', 3);
    points_table.insert('>', 4);

    let mut close_to_open_states = HashMap::new();
    close_to_open_states.insert(')', '(');
    close_to_open_states.insert(']', '[');
    close_to_open_states.insert('}', '{');
    close_to_open_states.insert('>', '<');

    let mut open_to_closed_states = HashMap::new();
    open_to_closed_states.insert('(', ')');
    open_to_closed_states.insert('[', ']');
    open_to_closed_states.insert('{', '}');
    open_to_closed_states.insert('<', '>');

    let mut score_per_line: Vec<i64> = raw_input
        .lines()
        .map(|line| incomplete_score(line, &points_table, &close_to_open_states, &open_to_closed_states))
        .collect();

    score_per_line.retain(|x| *x != 0);
    score_per_line.sort();

    let result = score_per_line[(score_per_line.len() - 1)/2];
    result
}

fn incomplete_score(raw_line: &str, points_table: &HashMap<char, i64>, close_to_open_states: &HashMap<char, char>, open_to_closed_states: &HashMap<char, char>) -> i64{
    let mut open_states_queue = Vec::new();
    let open_values: Vec<char> = close_to_open_states.values().cloned().collect();
    for char in raw_line.chars(){
        if open_values.contains(&char){
            // opening new state
            open_states_queue.push(char)
        }else {
            // it has to be closing state
            let coresponding_open_state = close_to_open_states.get(&char).unwrap();
            let last_value_in_queue = open_states_queue[open_states_queue.len() - 1];

            if coresponding_open_state == &last_value_in_queue{
                // its ok closing the corresponding open values
                // remove last element from open_states_queue
                open_states_queue.remove(open_states_queue.len() - 1);
            }else {
                // problem - closing something that should not be closed!
                return 0 as i64
            }
        }

    }
    // line is incomplete, finish it:
    let mut missing_closing_states = Vec::new();
    for val in open_states_queue.iter().rev(){
        missing_closing_states.push(*open_to_closed_states.get(val).unwrap())
    }
    let score = _p2_score_per_line(&missing_closing_states, points_table);
    return score

}

fn _p2_score_per_line(added_values: &Vec<char>, points_table: &HashMap<char, i64>) -> i64{
    let mut total_score: i64 = 0;
    for value in added_values{
        total_score = total_score * 5;
        total_score = total_score + points_table.get(value).unwrap();
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(p1(raw_input()), 26397);
        assert_eq!(p2(raw_input()), 288957);
    }

    fn raw_input<'a>() -> &'a str {
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    }
}