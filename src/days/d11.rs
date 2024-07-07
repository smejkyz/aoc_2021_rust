use std::collections::HashMap;
use itertools::Itertools;
use crate::utils::{Array2D, parse_2d_input};

pub fn p1(raw_input: &str) -> i32{
    let mut octopus_array = parse_2d_input(raw_input);
    println!("it worsk : {:?}", octopus_array);
    let max_nb_steps = 100;
    let mut nb_flashes = 0;
    for step_id in 0..max_nb_steps{
        let flashes_per_turn = perform_one_step(&mut octopus_array);
        nb_flashes = nb_flashes + flashes_per_turn;
        println!("after {} steps nb flash: {}, octopus array: {:?}", step_id + 1, nb_flashes,  octopus_array);
    }

    nb_flashes
}

pub fn p2(raw_input: &str) -> i32{
    let mut octopus_array = parse_2d_input(raw_input);
    println!("it worsk : {:?}", octopus_array);
    let mut step_id = 0;
    loop{
        let flashes_per_turn = perform_one_step(&mut octopus_array);
        step_id = step_id + 1;
        println!("after {} steps nb flash: {}", step_id, flashes_per_turn);
        if flashes_per_turn == 10 * 10{
            break;
        }

    }

    step_id
}


fn perform_one_step(octopus_array: &mut Array2D) -> i32{
    //increase level of each octopus
    octopus_array.increase_by(1);
    //
    let mut already_flashed: Vec<(usize, usize)> = Vec::new();
    let mut octopus_that_flashes = find_octopus_that_flashes(&octopus_array, &already_flashed);

    while octopus_that_flashes.len() > 0{
        already_flashed.append(&mut octopus_that_flashes.clone()); // todo: why is there &mut and clone?
        // each octopus that flashed increase by 1 its neighbours
        for (i, j) in octopus_that_flashes{
            let _8_neighbours = octopus_array.get_8_neighbours_coor(i, j);
            for (neighbour_i, neighbour_j) in _8_neighbours{
                octopus_array.set(neighbour_i, neighbour_j, octopus_array.get(neighbour_i, neighbour_j) + 1);
            }
        }
        octopus_that_flashes = find_octopus_that_flashes(&octopus_array, &already_flashed);
        //println!("it worsk : {:?}", octopus_array);
    }
    // count flash and
    let mut nb_flash = 0;
    for (i, j) in (0..octopus_array.get_rows()).cartesian_product(0..octopus_array.get_cols()) {
        if octopus_array.get(i, j) > 9{
            nb_flash += 1;
            octopus_array.set(i, j, 0);
        }
    }
    nb_flash
}

fn find_octopus_that_flashes(octopus_array: &Array2D, already_flashed: &Vec<(usize, usize)>) -> Vec<(usize, usize)>{
    let mut indices = Vec::new();
    for (i, j) in (0..octopus_array.get_rows()).cartesian_product(0..octopus_array.get_cols()){
        if octopus_array.get(i, j) > 9 && !already_flashed.contains(&(i,j)){
            indices.push((i,j));
        }
    }
    indices
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps(){
        let mut _input = parse_2d_input(raw_input());
        let exp_result = results();
        let after_one_step = perform_one_step(&mut _input);
        assert_eq!(_input, exp_result[0]);
        _input.display();
        println!("something");
        exp_result[0].display();
        perform_one_step(&mut _input);
        assert_eq!(_input, exp_result[1]);

        perform_one_step(&mut _input);
        assert_eq!(_input, exp_result[2]);
    }

    #[test]
    fn test_dummy(){
        let mut _input = parse_2d_input(raw_input_dummy());
        perform_one_step(&mut _input);
        _input.display()
    }

    #[test]
    fn it_works() {
        assert_eq!(p1(raw_input()), 1656);
        assert_eq!(p2(raw_input()), 195);
    }

    fn raw_input<'a>() -> &'a str {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }

    fn results<'a>() -> Vec<Array2D> {
        let after_step_1 = parse_2d_input("6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637");

        let after_step_2 = parse_2d_input("8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848");

        let after_step_3 = parse_2d_input("0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000");

        let after_step_4 = parse_2d_input("2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211");

        vec![after_step_1, after_step_2, after_step_3, after_step_4]
    }
    fn raw_input_dummy<'a>() -> &'a str {
        "11111
19991
19191
19991
11111"
    }
}