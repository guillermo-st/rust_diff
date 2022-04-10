use std::cmp;
use std::fmt;

#[derive(Debug)]
struct Lcs<'t> {
    grid: Vec<Vec<u8>>,
    first_sequence : &'t[String],
    second_sequence : &'t[String],
}

fn diff_rec(lcs: &Lcs, i: usize, j: usize){
    if i>0 && j>0 && lcs.first_sequence[i-1] == lcs.second_sequence[j-1]{
        diff_rec(lcs, i-1, j-1);
        println!("  {}", lcs.first_sequence[i-1]);
    }
    else if j>0 && (i==0 || lcs.grid[i][j-1] >= lcs.grid[i-1][j]){
        diff_rec(lcs, i, j-1);
        println!("> {}", lcs.second_sequence[j-1]);
    }
    else if i>0 && (j==0 || lcs.grid[i][j-1] < lcs.grid[i-1][j]){
        diff_rec(lcs, i-1, j);
        println!("< {}", lcs.first_sequence[i-1]);
    }
    else{
        println!();
    }
}

impl Lcs<'_> {
    fn new<'t>(first_sequence: &'t[String], second_sequence: &'t[String]) -> Lcs<'t> {
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

fn main() {
    let hello1 = vec![
        String::from("hello"),
        String::from("horrible"),
        String::from("world"),
        String::from("!"),
    ];
    let hello2 = vec![
        String::from("hello"),
        String::from("beautiful"),
        String::from("world"),
        String::from("!"),
    ];

    let a_grid = Lcs::new(&hello1, &hello2);
    println!("{}", a_grid);

    diff_rec(&a_grid, a_grid.first_sequence.len(), a_grid.second_sequence.len())
}
