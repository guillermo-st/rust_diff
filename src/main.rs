use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod lcs;

///Return a Result<Vec<String>> which contents are all the lines in the # filename file in abscence of error.
fn read_file_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("[ERR] Invalid number of arguments to generate diff. Usage: $ diff <filename1> <filename2>");
        return Err("[ERR] Invalid number of arguments to generate diff.");
    }

    let first_filename = &args[1].as_str();
    let second_filename = &args[2].as_str();

    let first_sequence = read_file_lines(first_filename);
    let second_sequence = read_file_lines(second_filename);

    match (first_sequence, second_sequence) {
        (Ok(first_seq), Ok(sec_seq)) => {
            let longest_common_sequence = lcs::Lcs::new(&first_seq, &sec_seq);
            longest_common_sequence.diff();
            Ok(())
        }

        (Err(e), _) => {
            eprintln!("[ERR] Unable to read first file to generate diff. {}", e);
            Err("[ERR] Invalid number of arguments to generate diff.")
        }
        (_, Err(e)) => {
            eprintln!("[ERR] Unable to read second file to generate diff. {}", e);
            Err("[ERR] Invalid number of arguments to generate diff.")
        }
    }
}
