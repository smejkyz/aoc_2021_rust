
const W_DAY: usize = 10;
const W_PART: usize = 10;

fn mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn measure_run<S: ?Sized, T, F: Fn(&S) -> T>(f: &F, input: &S) -> f64 {
    let start = std::time::SystemTime::now();
    _ = f(input);
    let duration = start.elapsed().unwrap();
    duration.as_secs_f64()
}

pub fn benchmark_run<S: ?Sized, T, F: Fn(&S) -> T>(f: F, input: &S) -> f64 {
    let first_run = measure_run(&f, input);
    let n = (1. / first_run) as i32;
    if n <= 1 || first_run < 0.000001 {
        return first_run;
    }
    let mut run_times = vec![];
    for _ in 0..n {
        run_times.push(measure_run(&f, input));
    }
    mean(&run_times)
}

pub fn print_header() {
    print!("{:<w$}", "day", w = W_DAY);
    print!("{:<w$}", "part 1", w = W_PART);
    print!("{:<w$}", "part 2", w = W_PART);
    println!();
    println!("{:-<w$}", "", w = W_DAY + W_PART * 2);
}

pub fn print_day(day: u8, p1: f64, p2: f64) {
    print!("{:<w$}", format!("day {:02}", day), w = W_DAY);

    let mut p1_dur = format!("{:.3}", p1 * 1000.).to_string();
    p1_dur = format!("{} ms", &p1_dur[..5]);
    print!("{:<w$}", p1_dur, w = W_PART);

    let mut p2_dur = format!("{:.3}", p2 * 1000.).to_string();
    p2_dur = format!("{} ms", &p2_dur[..5]);
    println!("{:<w$}", p2_dur, w = W_PART);
}


#[derive(Debug)]
pub struct Array2D {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>,
}

impl Array2D {
    // Constructor to create a new 2D array with default value
    pub fn new(rows: usize, cols: usize, default_value: i32) -> Self {
        let data = vec![vec![default_value; cols]; rows];
        Array2D { rows, cols, data }
    }

    pub fn get_rows(&self) -> usize{
        self.rows
    }

    pub fn get_cols(&self) -> usize{
        self.cols
    }
    // Method to get a value at a specific position
    pub fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row][col]
        // if row < self.rows && col < self.cols {
        //     Some(self.data[row][col])
        // } else {
        //     None
        // }
    }

    pub fn get_4_neighbours(&self, row: usize, col: usize) -> Vec<i32>{
        let mut result: Vec<i32> = Vec::new();
        if row > 0{
            // add upper
            result.push(self.get(row-1, col));
        }
        if row < self.rows - 1{
            // add below:
            result.push(self.get(row+1, col))
        }
        if col > 0{
            //add left
            result.push(self.get(row, col -1))
        }
        if col < self.cols - 1{
            result.push(self.get(row, col +1))
        }
        result
    }

    pub fn get_4_neighbours_coor(&self, row: usize, col: usize) -> Vec<(usize, usize)>{
        let mut result: Vec<(usize, usize)> = Vec::new();
        if row > 0{
            // add upper
            result.push((row-1, col));
        }
        if row < self.rows - 1{
            // add below:
            result.push((row+1, col))
        }
        if col > 0{
            //add left
            result.push((row, col -1))
        }
        if col < self.cols - 1{
            result.push((row, col +1))
        }
        result
    }

    // Method to set a value at a specific position
    fn set(&mut self, row: usize, col: usize, value: i32) -> Result<(), &'static str> {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    // Method to display the 2D array
    pub fn display(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }

    // Method to push a new row into the 2D array
    pub fn push(&mut self, new_row: Vec<i32>) -> Result<(), &'static str> {
        if new_row.len() == self.cols || self.rows == 0 {
            if self.rows == 0{
                self.cols = new_row.len();
            }
            self.data.push(new_row);
            self.rows += 1;
            Ok(())
        } else {
            Err("Length of the new row does not match the number of columns")
        }
    }
}
