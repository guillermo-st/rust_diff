use std::cmp;
use std::fmt;

mod diff;

#[derive(Debug)]
///Represents the "Longest Common Subsequence" operation between two String vectors.
pub struct Lcs<'t> {
    ///Matrix filled on Lcs construction, used to calculate the common sequences.
    grid: Vec<Vec<u8>>,
    first_sequence: &'t [String],
    second_sequence: &'t [String],
}

impl Lcs<'_> {
    ///Returns a new Lcs with the provided String sequences.
    ///
    /// # Arguments
    ///
    /// * `first_sequence` - A reference to a vector of Strings, with equal dimention to the resulting Lcs matrix columns.
    /// * `second_sequence` - A reference to a vector of Strings, with equal dimention to the resulting Lcs matrix rows.
    pub fn new<'t>(first_sequence: &'t [String], second_sequence: &'t [String]) -> Lcs<'t> {
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
            second_sequence,
        }
    }

    ///Prints a diff based on the sequences provided on construction, using the Lcs matrix to calculate it.
    pub fn diff(&self) {
        diff::diff_rec(
            &self.grid,
            self.first_sequence,
            self.second_sequence,
            self.first_sequence.len(),
            self.second_sequence.len(),
        )
    }
}

///Implementation of the "Display" trait to allow for printing Lcs to stdout.
impl fmt::Display for Lcs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid_string = String::new();

        for i in 0..self.grid.len() {
            grid_string.push_str(format!("{:?}\n", self.grid[i]).as_str());
        }
        write!(f, "{}", grid_string)
    }
}
