mod bytes;

// use std::env;
use crate::bytes::process_bytes;

fn main() {
    // let args: Vec<String> = env::args().collect();
    //
    // let input_file = &args[1];
    // let output_file = &args[2];
    //
    // process_csv(input_file, output_file)

    process_bytes();
}