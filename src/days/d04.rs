use ahash::RandomState;
use crate::utils::Array2D;

pub fn p1(raw_input: &str) -> i32{
    let (numbers, mut boards) = parse_input(raw_input);
    for chosen_number in numbers {
        // update boards - replace chosen_number with -1
        for board in &mut boards{
            board.replace(chosen_number, -1);
            if board_is_complete(board){
                // compute result
                let board_state_value = compute_board_state_value(board);
                let result =  chosen_number * board_state_value;
                println!("result, {}", result);
                return result
            }
        }
    }
    println!("");
    0
}

pub fn p2(raw_input: &str) -> i32{
    let (numbers, mut boards) = parse_input(raw_input);
    for chosen_number in numbers {
        // update boards - replace chosen_number with -1
        let mut board_id = 0;
        let mut boards_to_remove = Vec::new();
        println!("We have {} boards", boards.len());
        for board in &mut boards{
            board.replace(chosen_number, -1);
            if board_is_complete(board){
                // remove board from list
                boards_to_remove.push(board_id);
            }
            board_id += 1;
        }
        if boards.len() == 1 && boards_to_remove.len() == 1{
            let board_state_value = compute_board_state_value(&boards[0]);
            let result =  chosen_number * board_state_value;
            println!("result, {}", result);
            return result
        }
        boards_to_remove.reverse();
        println!("Removing boards {:?}", boards_to_remove);
        for _id in boards_to_remove{
            boards.remove(_id);
        }


    }

    0
}

fn board_is_complete(board: &Array2D) -> bool{
    // fist check rows:
    let bingo_vector = vec![-1; board.get_rows()];
    for row in 0..board.get_rows(){
        let row_data = board.get_row(row);
        if row_data == bingo_vector{
            return true
        }
    }

    for col in 0..board.get_cols(){
        let col_data = board.get_column(col);
        if col_data == bingo_vector{
            return true
        }
    }
    false
}

fn compute_board_state_value(board: &Array2D) -> i32{
    board.sum() + board.count(-1)
}
#[derive(Debug)]
pub struct Board {
    values: Array2D,
    has_been_drawn: Array2D,
}

fn parse_input(raw_input: &str) -> (Vec<i32>, Vec<Array2D>){
    let (numbers, other) = raw_input.split_once("\n\n").unwrap();

    let numbers_sequnce: Vec<i32> = numbers.split(",").map(|line| line.parse::<i32>().unwrap()).collect();
    let mut raw_bingo_cards: Vec<Array2D> = Vec::new();
    for raw_bingo in other.split("\n\n"){
        let mut bingo_card = Array2D::new(0, 0, 0);
        for line in raw_bingo.split("\n"){
            let int_line = parse_bingo_line(line);
            bingo_card.push(int_line);

        }
        raw_bingo_cards.push(bingo_card);
    }


   let result =  (numbers_sequnce, raw_bingo_cards);
    result
}

fn parse_bingo_line(raw_line: &str) -> Vec<i32>{
    let mut int_line: Vec<i32> = Vec::new();
    for part in raw_line.split(" "){
        if part.len() > 0{
            let part_as_int = part.parse::<i32>().unwrap();
            int_line.push(part_as_int);
        }

    }
    int_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(p1(raw_input()), 4512);
        assert_eq!(p2(raw_input()), 1924);
    }

    fn raw_input<'a>() -> &'a str {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
    }
}