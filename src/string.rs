use std::fs;

#[inline]
pub fn process_csv2(input_file: &str, output_file: &str) {
    let mut file_contents = fs::read_to_string(input_file).expect("Unable to read file");

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
