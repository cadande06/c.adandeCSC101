use std::io;
fn main() {
    println!("Hello. Here is a list of equations:\nT for Area of Trapezium formula = (height/2)*(base1+base2)\nR for Area of the rhombus formula = 1/2 * diagonal1 * diagonal2\nP for Area of Parallelogram formula = base * altitude\nC for Area of Cube formula = 6*(length of the side)^2\nL for Volume of Cylinder formula = pi*radius^2*height\nHow many calculations do you want to perform?");
    let mut input1=String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let mut amount:u32=input1.trim().parse().expect("Failed to convert input");
    let choices = ['T','R','P','C','L'];
     for a in range 0..amount{
        println!("From the list given above, please choose your equation.");
        
        let input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let eqn:&str=input.trim().parse().expect("Failed to parse input");
        
        if choices.contains(eqn){
            if eqn='T'{
                
            }
        }
        else{
            println!("Sorry, your choice is not in the list of equations.");
        }
     }

}

fn trap(height:i32,base1:i32,base2:i32){
    return (height/2)*(base1+base2);
}
