use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;

mod lcs;

fn read_file_lines(filename: &str) -> io::Result<Vec<String>>{
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        panic!("[ERR] Invalid number of arguments to generate diff. Usage: $ diff <filename1> <filename2>");
    }

    let first_filename = &args[1].as_str();
    let second_filename = &args[2].as_str();

    let first_sequence = read_file_lines(first_filename);
    match first_sequence{

        Ok(first_seq) => {

            let second_sequence = read_file_lines(second_filename);
            match second_sequence{
                Ok(second_seq) => {

                    let longest_common_sequence = lcs::Lcs::new(&first_seq, &second_seq);
                    longest_common_sequence.diff();

                },
                Err(e) => panic!("[ERR] Unable to open file {}. {:?}", second_filename, e.to_string())
            };

        },
        Err(e) => panic!("[ERR] Unable to open file {}. {:?}", first_filename, e.to_string())
    };
}
