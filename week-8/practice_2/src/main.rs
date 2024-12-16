use std::io::Read;
fn main() {
    let mut file = std::fs::File::open("../practice_1/data.txt").unwrap();//unwrap is used to handle the result. Expect can also be used. If the opreation succeeds, unwrap extracts the file handle
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();//Reads the entire contents of the file into the contents string.
    print!("{}",contents);
}
