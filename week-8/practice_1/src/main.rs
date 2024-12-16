use std::io::Write;//to be able to use write_all methods
fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("/Users/chiomadande/Documents/c.adandeCSC101/week-8/practice_1/data.txt").expect("create failed");//creates a new text file. If this file already exists, it will overwrite the file
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed");
    // write_all => writes the data into the file
    // .as_bytes() => converts the data into a byte array because thats what file I/O works with
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
}
 // if you want to preserve the old code, use this =>

// use std::fs::OpenOptions;
// use std::io::Write;

// fn main() {
//     let announce = "Week 9 - Rust File Input & Output\n";
//     let dept = "Department of Computer Science";

//     // Open the file in append mode, creating it if it doesn't exist
//     let mut file = OpenOptions::new()
//         .append(true) // Enable appending
//         .create(true) // Create the file if it doesn't exist
//         .open("data.txt")
//         .expect("failed to open file");

//     // Write data to the file
//     file.write_all("Welcome to Rust Programming\n".as_bytes())
//         .expect("write failed");
//     file.write_all(announce.as_bytes()).expect("write failed");
//     file.write_all(dept.as_bytes()).expect("write failed");
// }
