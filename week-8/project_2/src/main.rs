use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // creating txt file
    let mut file = OpenOptions::new().append(true).create(true).open("/Users/chiomadande/Documents/c.adandeCSC101/week-8/project_2/target/sims.txt").expect("failed to open sims file");
    writeln!(file, "\n{:^65}", "STUDENT MANAGEMENT INFORMATION SYSTEM").expect("failed to add input");


    // creating vectors
    let head ="PAU SIMS";
    let headers = ["STUDENT NAME", "MATRIC. NUMBER", "DEPARTMENT", "LEVEL"];

    let stud_info = vec![
        ["Oluchi Mordi", "ACC10211111", "Accounting", "300"],
        ["Adams Aliyu", "ECO10110101", "Economics", "100"],
        ["Shania Bolade", "CSC10328828", "Computer", "200"],
        ["Adekunle Gold", "EEE11020202", "Electrical", "200"],
        ["Blanca Edemoh", "MEE1022001", "Mechanical", "100"]
    ];

    // adding headers to the file
    writeln!(file, "{:^65}", head).expect("failed to add input");
    writeln!(file, "\n{:<20}{:<20}{:<20}{:<5}", headers[0], headers[1], headers[2], headers[3]).expect("failed to add input");
    writeln!(file, "{:-<65}", "").expect("failed to add input to file");
    
    // adding the student info
    for i in stud_info{
        writeln!(file, "\n{:<20}{:<20}{:<20}{:>5}", i[0], i[1], i[2], i[3]).expect("failed to add input");
        writeln!(file, "{:-<65}", "").expect("failed to add input to file");
    }
    println!("STUDENT INFO COMPLETE! The file is located in the target folder of this project.");
}
