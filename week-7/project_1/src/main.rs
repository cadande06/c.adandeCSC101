use std::io;
fn main() {
    // introductory speech
    println!("*************** Public Service APS Level Checker ***************");
    println!("Hello! Here are the lists of job positions (Please, type in the appropriate letters to represent them): ");
    println!("\n*********** JOB POSITIONS ***********\n'O' - Office Administrator\n'A' - Academic\n'L' - Lawyer\n'T' - Teacher\nJob: ");
    
    // vectors
    let job_arr = vec!['O', 'A', 'L', 'T'];

    // getting inputs
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("FAILED TO READ INPUT");
    let job: char = input1.trim().parse().expect("FAILED TO PARSE INPUT");

    //making sure they put in the write values
    if job_arr.contains(&job) {
        // CHECKING....
        if job=='O'{
            off_admin();
        }
        else if job == 'A'{
            academic();
        }
        else if job == 'L'{
            lawyer();
        }
        else if job == 'T'{
            teacher();
        }
    } else {
        println!("Please, your input is not in the list given above.");
    }
}

fn off_admin() {
    // JOB LEVEL
    println!("\nHere's a list of Office Administrator job levels and their experience levels in years. Please, choose from the options which job level you are in.");
    println!("'I' - Intern => 1-2\n'A' - Administrator => 3-5\n'S' - Senior Administrator => 5-8\n'O' - Office Manager => 8-10\n'D' - Director => 10-13\n'C' - CEO => 13");
    let lev = vec!['I', 'A', 'S', 'O', 'D', 'C'];

    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("FAILED TO READ INPUT");
    let job: char = input1.trim().parse().expect("FAILED TO PARSE INPUT");

    // CHECKING
    if lev.contains(&job) {
        if job == 'I' {
            println!("From the data you provided, you hold a position of APS 1-2.");
        } else if job == 'A' {
            println!("From the data you provided, you hold a position of APS 3-5.");
        } else if job == 'S' {
            println!("From the data you provided, you hold a position of APS 5-8.");
        } else if job == 'O' {
            println!("From the data you provided, you hold a position of EL 1 8-10.");
        } else if job == 'D' {
            println!("From the data you provided, you hold a position of EL 2 10-13.");
        } else if job == 'C' {
            println!("From the data you provided, you hold a position of SES");
        }
    } else {
        println!("Your choice is not in the list.");
    }
}

fn academic(){
    // JOB LEVEL
    println!("\nHere's a list of Academic job levels and their experience levels in years. Please, choose from the options which job level you are in.");
    println!("'R' - Research Assistant => 3-5\n'P' - PhD Candidate => 5-8\n'O' - Post-Doc Researcher => 8-10\n'S' - Senior Lecturer => 10-13\n'D' - Dean => 13");
    let lev = vec!['R', 'P', 'O', 'S', 'D'];

    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("FAILED TO READ INPUT");
    let job: char = input1.trim().parse().expect("FAILED TO PARSE INPUT");

    // CHECKING
    if lev.contains(&job) {
        if job == 'R' {
            println!("From the data you provided, you hold a position of APS 3-5.");
        } else if job == 'P' {
            println!("From the data you provided, you hold a position of APS 5-8.");
        } else if job == 'O' {
            println!("From the data you provided, you hold a position of EL 1 8-10.");
        } else if job == 'S' {
            println!("From the data you provided, you hold a position of EL 2 10-13.");
        } else if job == 'D' {
            println!("From the data you provided, you hold a position of SES");
        }
    } else {
        println!("Your choice is not in the list.");
    }

}

fn lawyer() {
    // JOB LEVEL
    println!("\nHere's a list of Lawyer job levels and their experience levels in years. Please, choose from the options which job level you are in.");
    println!("'P' - Paralegal => 1-2\n'J' - Junior Associate => 3-5\n'A' - Associate => 5-8\n'O' - Senior Associate 1-2 => 8-10\n'T' - Senior Associate 3-4 => 10-13\n'N' - Partner => 13");
    let lev = vec!['P', 'J', 'A', 'O', 'T', 'N'];

    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("FAILED TO READ INPUT");
    let job: char = input1.trim().parse().expect("FAILED TO PARSE INPUT");

    // CHECKING
    if lev.contains(&job) {
        if job == 'P' {
            println!("From the data you provided, you hold a position of APS 1-2.");
        } else if job == 'J' {
            println!("From the data you provided, you hold a position of APS 3-5.");
        } else if job == 'A' {
            println!("From the data you provided, you hold a position of APS 5-8.");
        } else if job == 'O' {
            println!("From the data you provided, you hold a position of EL 1 8-10.");
        } else if job == 'T' {
            println!("From the data you provided, you hold a position of EL 2 10-13.");
        } else if job == 'N' {
            println!("From the data you provided, you hold a position of SES");
        }
    } else {
        println!("Your choice is not in the list.");
    }
}

fn teacher(){
// JOB LEVEL
println!("\nHere's a list of Teacher job levels and their experience levels in years. Please, choose from the options which job level you are in.");
println!("'P' - Placement => 1-2\n'C' - Classroom Teacher => 3-5\n'S' - Snr Teacher => 5-8\n'L' - Leading Teacher => 8-10\n'D' - Deputy Principal => 10-13\n'P' - Principal => 13");
let lev = vec!['P', 'C', 'S', 'L', 'D', 'P'];

let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("FAILED TO READ INPUT");
    let job: char = input1.trim().parse().expect("FAILED TO PARSE INPUT");

    // CHECKING
    if lev.contains(&job) {
        if job == 'P' {
            println!("From the data you provided, you hold a position of APS 1-2.");
        } else if job == 'C' {
            println!("From the data you provided, you hold a position of APS 3-5.");
        } else if job == 'S' {
            println!("From the data you provided, you hold a position of APS 5-8.");
        } else if job == 'L' {
            println!("From the data you provided, you hold a position of EL 1 8-10.");
        } else if job == 'D' {
            println!("From the data you provided, you hold a position of EL 2 10-13.");
        } else if job == 'P' {
            println!("From the data you provided, you hold a position of SES");
        }
    } else {
        println!("Your choice is not in the list.");
    }
}
