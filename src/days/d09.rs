use ahash::HashSet;
use bstr::ByteSlice;
use crate::utils::{Array2D, print_day, parse_2d_input};
use itertools::Itertools;

pub fn p1(raw_input: &str) -> i32 {
    let input_matrx = parse_2d_input(raw_input);
    let low_points = find_low_points(&input_matrx);
    let result = low_points.iter().map(|(i, j) | input_matrx.get(*i, *j) + 1).sum();
    assert_eq!(result, 570); // 15 for test, 570 run
    result
}

fn find_low_points(input_matrix: &Array2D) -> Vec<(usize, usize)>{
    let mut result = Vec::new();
    for (i, j) in (0..input_matrix.get_rows()).cartesian_product(0..input_matrix.get_cols()){
        let neighbours = input_matrix.get_4_neighbours(i, j);
        let value = input_matrix.get(i, j);
        if &value < neighbours.iter().min().unwrap(){
            result.push((i, j));
        }
    }
    result
}

pub fn p2(raw_input: &str) -> i32 {
    let input_matrx = parse_2d_input(raw_input);
    let low_points = find_low_points(&input_matrx);
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|point| compute_basin_size(*point, &input_matrx))
        //.collect::<Vec<_>>()
        .sorted()
        .collect();

    basin_sizes.reverse();
    let result = (basin_sizes[0] * basin_sizes[1] * basin_sizes[2]) as i32;
    assert_eq!(result, 899392);
    result
}


fn compute_basin_size(point: (usize, usize), input_matrix: &Array2D) -> usize{
    //starting from a low point expand until now expansion is possible. Expand to neighbours with higher value, not 9
    let mut basin: Vec<(usize, usize)> = vec![point];
    let mut queeue: Vec<(usize, usize)> = vec![point];
    while queeue.len() > 0{
        let something = queeue.pop().unwrap();
        let val_something = input_matrix.get(something.0, something.1);
        for neighbour_coor in input_matrix.get_4_neighbours_coor(something.0, something.1){
            let val_neighbour = input_matrix.get(neighbour_coor.0, neighbour_coor.1);
            if val_neighbour > val_something && val_neighbour < 9 && !basin.contains(&neighbour_coor){
                queeue.push(neighbour_coor);
                basin.push(neighbour_coor);
            }
        }
    }
    basin.len()
}
