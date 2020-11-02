use std::{env,fs};
use bstr::{ByteSlice,ByteVec};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = &args[1];
    let output_file = &args[2];

    let mut file_contents = fs::read(input_file).expect("Unable to read file");

    file_contents.replace_range(..1, "<<< HEAD >>>");
    let t1 = file_contents.replace("\r\n\"", "\r\n<<< START >>>")
        .replace("\",\"", "<<< INNER >>>")
        .replace("\"\r\n", "<<< END >>>")
        .replace("\"", "\"\"")
        .replace("<<< END >>>", "\"\r\n")
        .replace("<<< INNER >>>", "\",\"")
        .replace("<<< START >>>", "\"")
        .replace("<<< HEAD >>>", "\"");

    fs::write(output_file, t1).expect("Unable to write file");
}