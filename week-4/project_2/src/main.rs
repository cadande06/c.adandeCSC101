use std::io;

fn main() {
    //getting employee experience and age
    let mut exp = String::new();
    let mut age = String::new();
    let mut inc = 0;
    println!("Please type in your experience level (e.g experienced or inexperienced).");
    io::stdin().read_line(&mut exp).expect("Failed to read experience");
    let exp:&str=exp.trim();
    println!("Please type in your age (e.g 35).");
    io::stdin().read_line(&mut age).expect("Failed to read age");
    let age:i32=age.trim().parse().expect("Not a valid number");

    if exp == "experienced" && age>=40{
        inc+=1560000;
    } else if exp == "experienced" && age>=30 && age<40{
        inc+=1480000;
    }else if exp == "experienced" && age<28{
        inc+=1300000;
    }else if exp == "inexperienced"{
        inc+=100000;
    }
    let annual_inc:i32=inc*12;
    println!("Based on your experience level and age, your annual incentive is N{annual_inc}.");
}
