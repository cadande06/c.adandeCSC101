use std::io;
fn main() {
    // getting the number of orders
    println!("***************** Hello! Welcome to our restaurant *****************\n                How much food would you like to buy?");
    let mut input1=String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let quantity:i32=input1.trim().parse().expect("Please input a number");
    
    // getting the orders
    println!("*********************** Here is our menu ***************************\nType:\nP for Poundo Yam/Edinkaiko Soup at N3,200.00\nF for Fried Rice & Chicken at N3,000.00\nA for Amala & Ewedu Soup at N2,500.00\nE for Eba & Egusi Soup at N2,000.00\nW for White Rice & Stew at N2,500.00");
    let menu:[&str;5]=["P", "F", "A", "E", "W"];
    let mut price:f32=0.0;
    let mut q:i32 = 0;
    let mut choices=vec![];
    println!("What are your choices?");
    while q<quantity{
        
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let choice=input.trim();
        if menu.contains(&choice){
            if choice=="P"{
                price+=3_200.0;
                choices.push("Poundo Yam/Edinkaiko Soup");
            }
            else if choice=="F"{
                price+=3_000.0;
                choices.push("Fried Rice & Chicken");
            } 
            else if choice=="A"{
                price+=2_500.0;
                choices.push("Amala & Ewedu Soup");
            }
            else if choice=="E"{
                price+=2_000.0;
                choices.push("Eba & Egusi Soup");
            }
            else if choice=="W"{
                price+=2_500.0;
                choices.push("White Rice & Stew");
            }
            q+=1;
            
        }
        else{
            println!("Sorry. That is not part of the menu. Please, pick a choice from the menu. Thank you.");
        }

    }
    if price>10_000.0{
        price*=0.95;
    }
    println!("Your order is {:?} and the total price for it is N{}\nEnjoy your meal!", choices, price);

    
    
    
    

    
    

    
    
    
    
}
