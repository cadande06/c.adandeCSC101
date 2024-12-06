use std::io;
fn main() {
    let mut name_vec=vec![];
    let mut years_vec:Vec<u32> = Vec::new();
    let mut highest:u32 = 0;
    let mut nameh:String="Empty".to_string();
    println!("\nERNST & YOUNG GLOBAL LIMITED NIGERIA");
    println!("Welcome to EY Nigeria! During this job interview, I would like to record name and years of programming experience of each applicant.");
    println!("Please, how many applicants are there?");
    let mut nu = String::new();
    io::stdin().read_line(&mut nu).expect("Failed to read input");
    let nu:u32=nu.trim().parse().expect("Failed to parse input");
    
    // getting info from applicants
    for n in 1..=nu{
    println!("\nApplicant {n} - Please, type in your first and last names:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name:String=name.trim().parse().expect("WRONG INPUT TYPE");

    println!("Please, type in your years of programming experience:");
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Failed to read input");
    let years:u32=years.trim().parse().expect("WRONG INPUT TYPE");

    name_vec.push(name);
    years_vec.push(years);
    }

    // checking for highest experience
    for i in 0..years_vec.len(){
        if years_vec[i]>0{
            highest = years_vec[i];
            nameh = name_vec[i].clone();
        }
    }

    println!("\nFrom my review of all job apllicants, the applicant with the highest programming experience is {:?} and has an experience of {:?}years.", nameh, highest);
    println!("\nThank you all for your time.");
}
