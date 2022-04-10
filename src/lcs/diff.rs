pub fn diff_rec(grid: &[Vec<u8>], first_seq: &[String], second_seq: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && first_seq[i - 1] == second_seq[j - 1] {
        diff_rec(grid, first_seq, second_seq, i - 1, j - 1);
        println!("  {}", first_seq[i - 1]);
    } else if j > 0 && (i == 0 || grid[i][j - 1] >= grid[i - 1][j]) {
        diff_rec(grid, first_seq, second_seq, i, j - 1);
        println!("> {}", second_seq[j - 1]);
    } else if i > 0 && (j == 0 || grid[i][j - 1] < grid[i - 1][j]) {
        diff_rec(grid, first_seq, second_seq, i - 1, j);
        println!("< {}", first_seq[i - 1]);
    } else {
        println!();
    }
}
