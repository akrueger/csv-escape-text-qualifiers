// use std::fs;
// use bstr::{B, ByteSlice};

use std::fs;

#[inline]
// pub fn process_csv(input_file: &str, output_file: &str) {
    // let file_contents = fs::read(input_file).expect("Unable to read file");
    // println!("{}", std::str::from_utf8(&file_contents).unwrap());
    // file_contents.replace_range(..1, "<<< HEAD >>>");
    // let t1 = file_contents.replace("\r\n\"", "\r\n<<< START >>>")
    //     .replace("\",\"", "<<< INNER >>>")
    //     .replace("\"\r\n", "<<< END >>>")
    //     .replace("\"", "\"\"")
    //     .replace("<<< END >>>", "\"\r\n")
    //     .replace("<<< INNER >>>", "\",\"")
    //     .replace("<<< START >>>", "\"")
    //     .replace("<<< HEAD >>>", "\"");

    // fs::write(output_file, t1).expect("Unable to write file");
// }

pub fn process_bytes() {
    // let s = "hello".as_bytes();
    // println!("{:?}", s);

    let file_contents = fs::read("./INDEX0001.csv").expect("Unable to read file");
    println!("{:?}", file_contents);
}
