//use std::io;
fn main(){
    println!("Hi");
    /*println!("Hi");
    let mut _price:f64=0.0;

    //Collecting Inputs
    let mut name=String::new();
    let mut input_2=String::new();
    let mut email=String::new();
    let mut input_4=String::new();
    let mut input_5=String::new();
    let mut input_6=String::new();
    let mut diagnosis=String::new();
    let mut village=String::new();

    println!("Please type your name");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Please type your date of birth");
    io::stdin().read_line(&mut input_2).expect("Failed to read input");
    let dob:u32=input_2.trim().parse().expect("Input not an integer");

    println!("Please type your email address");
    io::stdin().read_line(&mut email).expect("Failed to read input");
    
    println!("Please type your phone number");
    io::stdin().read_line(&mut input_4).expect("Failed to read input");
    let phone:u64=input_4.trim().parse().expect("Input not an integer");

    println!("Please type the number of siblings you have");
    io::stdin().read_line(&mut input_5).expect("Failed to read input");
    let siblings:u32=input_5.trim().parse().expect("Input not an integer");

    println!("Please type the number of children you have");
    io::stdin().read_line(&mut input_6).expect("Failed to read input");
    let children:u32=input_6.trim().parse().expect("Input not an integer");

    println!("Please type your medical diagnosis");
    io::stdin().read_line(&mut diagnosis).expect("Failed to read input");

    println!("Please type your village of residence");
    io::stdin().read_line(&mut village).expect("Failed to read input");

    
    for patient_no in 1..=100{
        let age:u32=2024-dob;
        let mut discount:f64 = 0.0;
        if diagnosis == "Alzheimer" && age > 50 && children > 4 && village == "Akpabom"{
            discount +=0.2*1_200_000.0;
            _price = 1_200_000.0-discount;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}\nDiscount: {discount}");
        } 
        else if diagnosis == "Arrhythmia" && age == 30 && siblings > 4 && village == "Ngbauji"{
            discount +=0.05*550_000.0;
            _price = 550_000.0-discount;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}\nDiscount: {discount}");
        } 
        else if diagnosis == "Chronic Kidney Disease" && age > 40 && children > 3 && siblings >3 && village == "Atabrikang"{
            discount +=0.15*1_500_000.0;
            _price = 1_500_000.0-discount;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}\nDiscount: {discount}");
        } 
        else if diagnosis == "Diabetes" && age > 28 && age < 45 && children >=2 && children <=4 && village == "Okorobilom"{
            discount +=0.1*800_000.0;
            _price = 800_000.0-discount;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}\nDiscount: {discount}");
        } 
        else if diagnosis == "Arthritis" && age > 58 && children > 5 && children >5 && village == "Emeremen"{
            discount +=0.1*450_000.0;
            _price = 450_000.0-discount;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}\nDiscount: {discount}");
        }
        else if diagnosis == "Alzheimer"{
            _price = 1_200_000.0;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
        }
        else if diagnosis == "Arrhythmia"{
            _price = 550_000.0;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
        }
        else if diagnosis == "Chronic Kidney Disease"{
            _price = 1_500_000.0;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
        }
        else if diagnosis == "Diabetes"{
            _price = 800_000.0;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
        }
        else if diagnosis == "Arthritis"{
            _price = 450_000.0;
            println!("Name: {name}\nPatient number: {patient_no}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
        }
        
    }
    
    if diagnosis == "Alzheimer"{
        let _price:f32 = 1_200_000.0;
        println!("Name: {name}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
    }
    else if diagnosis == "Arrhythmia"{
        let _price = 550_000.0;
        println!("Name: {name}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
    }
    else if diagnosis == "Chronic Kidney Disease"{
        let _price = 1_500_000.0;
        println!("Name: {name}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
    }
    else if diagnosis == "Diabetes"{
        let _price = 800_000.0;
        println!("Name: {name}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
    }
    else if diagnosis == "Arthritis"{
        let _price = 450_000.0;
        println!("Name: {name}\nDate of Birth: {dob}\nEmail Address: {email}\nPhone Number: {phone}\nNumber of Siblings: {siblings}\nNumber of Children: {children}\nMedical Diagnosis: {diagnosis}\nVillage of Residence: {village}\n_price of Treatment: {_price}");
    }*/
}