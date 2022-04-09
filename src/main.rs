use std::cmp;
use std::fmt;

#[derive(Debug)]
struct LCS {
    grid: Vec<Vec<u8>>,
}

impl LCS {
    fn new(first_sequence: &[String], second_sequence: &[String]) -> LCS {
        let first_len = first_sequence.len();
        let sec_len = second_sequence.len();
        let mut grid = vec![vec![0; sec_len + 1]; first_len + 1];

        for i in 0..first_len {
            for j in 0..sec_len {
                if first_sequence[i] == second_sequence[j] {
                    grid[i + 1][j + 1] = grid[i][j] + 1;
                } else {
                    grid[i + 1][j + 1] = cmp::max(grid[i + 1][j], grid[i][j + 1]);
                }
            }
        }
        LCS { grid }
    }
}

impl fmt::Display for LCS {
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
        String::from("world"),
        String::from("!"),
    ];
    let hello2 = vec![
        String::from("hello"),
        String::from("beautiful"),
        String::from("world"),
        String::from("!"),
    ];

    let a_grid = LCS::new(&hello1, &hello2);
    println!("{}", a_grid);
}
