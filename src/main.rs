use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = &args[1];
    let output_file = &args[2];

    let mut file_contents = fs::read_to_string(input_file).expect("Unable to read file");

    file_contents.replace_range(..1, "<**HEAD**>");
    let t1 = file_contents.replace("\r\n\"", "\r\n<**START**>")
        .replace("\",\"", "<**INNER**>")
        .replace("\"\r\n", "<**END**>")
        .replace("\n", "<**LF**>")
        .replace("\"", "\"\"")
        .replace("<**LF**>", "\n")
        .replace("<**END**>", "\"\r\n")
        .replace("<**INNER**>", "\",\"")
        .replace("<**START**>", "\"")
        .replace("<**HEAD**>", "\"");

    // let re1 = Regex::new("^\"").unwrap();
    // let re2 = Regex::new("(\r\n\")").unwrap();

    // let t1 = re1.replace_all(&file_contents, "<**HEAD**>");
    // let t2 = re2.replace_all(&*t1, "\r\n<**START**>");
    // let t3 = t2.replace("\",\"", "<**INNER**>");
    // let t4 = t3.replace("\"\r\n", "<**END**>");
    // let t5 = t4.replace("\n", "<**LF**>");
    // let t6 = t5.replace("\"", "\"\"");
    // let t7 = t6.replace("<**LF**>", "\n");
    // let t8 = t7.replace("<**END**>", "\"\r\n");
    // let t9 = t8.replace("<**INNER**>", "\",\"");
    // let t10 = t9.replace("<**START**>", "\"");
    // let t11 = t10.replace("<**HEAD**>", "\"");

    fs::write(output_file, t1).expect("Unable to write file");
}