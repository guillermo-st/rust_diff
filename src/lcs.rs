use std::cmp;
use std::fmt;

#[derive(Debug)]
pub struct Lcs<'t> {
    grid: Vec<Vec<u8>>,
    first_sequence : &'t[String],
    second_sequence : &'t[String],
}

pub fn diff_rec(grid:&Vec<Vec<u8>>, first_seq: &[String], second_seq: &[String], i: usize, j: usize){
    if i>0 && j>0 && first_seq[i-1] == second_seq[j-1]{
        diff_rec(grid, first_seq, second_seq, i-1, j-1);
        println!("  {}", first_seq[i-1]);
    }
    else if j>0 && (i==0 || grid[i][j-1] >= grid[i-1][j]){
        diff_rec(grid,first_seq, second_seq, i, j-1);
        println!("> {}", second_seq[j-1]);
    }
    else if i>0 && (j==0 || grid[i][j-1] < grid[i-1][j]){
        diff_rec(grid, first_seq, second_seq, i-1, j);
        println!("< {}", first_seq[i-1]);
    }
    else{
        println!();
    }
}

impl Lcs<'_> {
    pub fn new<'t>(first_sequence: &'t[String], second_sequence: &'t[String]) -> Lcs<'t> {
        let first_len = first_sequence.len();
        let sec_len = second_sequence.len();
        let mut grid = vec![vec![0; sec_len + 1]; first_len + 1];

        for i in 0..first_len {
            for (j, _) in second_sequence.iter().enumerate() {
                if first_sequence[i] == second_sequence[j] {
                    grid[i + 1][j + 1] = grid[i][j] + 1;
                } else {
                    grid[i + 1][j + 1] = cmp::max(grid[i + 1][j], grid[i][j + 1]);
                }
            }
        }
        Lcs {
            grid,
            first_sequence,
            second_sequence
        }
    }

    pub fn diff<'t>(&self)-> (){
        diff_rec(&self.grid, &self.first_sequence, &self.second_sequence, self.first_sequence.len(), self.second_sequence.len())
    }
}


impl fmt::Display for Lcs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid_string = String::new();

        for i in 0..self.grid.len() {
            grid_string.push_str(format!("{:?}\n", self.grid[i]).as_str());
        }
        write!(f, "{}", grid_string)
    }
}
