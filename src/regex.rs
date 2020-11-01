use std::fs;

#[inline]
pub fn process_csv1(input_file: &str, output_file: &str) {
    let mut file_contents = fs::read_to_string(input_file).expect("Unable to read file");

    let re1 = Regex::new("^\"").unwrap();
    let re2 = Regex::new("(\r\n\")").unwrap();

    let t1 = re1.replace_all(&file_contents, "<**HEAD**>");
    let t2 = re2.replace_all(&*t1, "\r\n<**START**>");
    let t3 = t2.replace("\",\"", "<**INNER**>");
    let t4 = t3.replace("\"\r\n", "<**END**>");
    let t5 = t4.replace("\"", "\"\"");
    let t6 = t5.replace("<**END**>", "\"\r\n");
    let t7 = t6.replace("<**INNER**>", "\",\"");
    let t8 = t7.replace("<**START**>", "\"");
    let t9 = t8.replace("<**HEAD**>", "\"");

    fs::write(output_file, t9).expect("Unable to write file");
}
