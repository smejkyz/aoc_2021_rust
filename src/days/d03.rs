use num::pow;

pub fn p1(raw_input: &str) -> i32 {
    let input = parse_input(raw_input);
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();
    for val in input.iter(){
        let ones: i32 = val.iter().sum();
        let zeros = val.len() as i32 - ones;
        if ones > zeros{
            gamma_rate.push(1);
            epsilon_rate.push(0);
        }else {
            gamma_rate.push(0);
            epsilon_rate.push(1);
        }
    }
    let gamma_rate_10 = binary_to_decimal(gamma_rate);
    let epsilon_rate_10 = binary_to_decimal(epsilon_rate);
    let p1 = gamma_rate_10 * epsilon_rate_10;
    println!("p1 {}", p1);
    p1
}

pub fn p2(raw_input: &str) -> i32{
    let mut input = parse_input(raw_input);
    let mut indicis_to_use: Vec<usize> = (0..input[0].len()).collect();
    let mut oxygen_generator_rating: Vec<i32> = Vec::new();
    for bit in input{
        let selected_bits: Vec<_> = indicis_to_use.iter().map(|&i| bit[i]).collect();
        let ones: i32 = selected_bits.iter().sum();
        let zeros = selected_bits.len() as i32 - ones;
        println!("a")
    }
    0
}

fn binary_to_decimal(binary_number: Vec<i32>) -> i32{
    let mut result = 0;
    for i in (0..binary_number.len()){
        let x_i = binary_number[i];
        let increment = x_i * pow(2, binary_number.len() - 1 - i);
        result += increment;
    }
    result
}


fn parse_input(raw_input: &str) -> Vec<Vec<i32>>{
    let mut data = Vec::new();
    for line in raw_input.lines(){
        let mut vec = Vec::new();
        for char in line.chars(){
            let a = char.to_string().parse::<i32>().unwrap();
            vec.push(a);

        }
        data.push(vec);
    }
    let mut data_transpose = vec![vec![0; data.len()]; data[0].len()];
    for i in 0..data.len(){
        for j in 0..data[0].len(){
            data_transpose[j][i] = data[i][j];
        }
    }
    data_transpose
}