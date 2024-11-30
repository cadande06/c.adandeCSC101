use std::io;
fn main() {
    // getting the amount of eqns
    println!("-----------------------------------------------CALCULATOR-----------------------------------------------");
    println!("Hello. Here is a list of equations:\nT for Area of Trapezium formula = (height/2)*(base1+base2)\nR for Area of the rhombus formula = 1/2 * diagonal1 * diagonal2\nP for Area of Parallelogram formula = base * altitude\nC for Area of Cube formula = 6*(length of the side)^2\nL for Volume of Cylinder formula = pi*radius^2*height\nHow many calculations do you want to perform?");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let amount: u32 = input1.trim().parse().expect("Failed to convert input");
    let choices: [char; 5] = ['T', 'R', 'P', 'C', 'L'];

    //  choosing equations
    let mut i = 0;
    while i < amount {
        println!("From the list given above, please choose your equation.");
        let mut input2 = String::new();
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input");
        let eqn: char = input2.trim().parse().expect("Failed to parse input");

        // making sure eqn is from the list
        if choices.contains(&eqn) {
            // checking which eqn to perform

            // trapezium area
            if eqn == choices[0] {
                let mut params: [f32; 3] = [1.0, 2.0, 3.0];
                println!("Please, input your height, base1 and base2 for the equation.");
                for i in 0..3 {
                    let mut param = String::new();
                    io::stdin()
                        .read_line(&mut param)
                        .expect("failed to read input");
                    let param: f32 = param.trim().parse().expect("failed to parse input");
                    params[i] = param;
                }
                // assign the inputs
                let height: f32 = params[0];
                let base1: f32 = params[1];
                let base2: f32 = params[2];
                // perform the calculations
                let area: f32 = trap(height, base1, base2);
                println!("Your equation is ({height}/2)*({base1}+{base2}) and the area of the trapezium is {area}cm^2");
            }
            // rhombus area
            else if eqn == choices[1] {
                let mut params: [f32; 2] = [1.0, 2.0];
                println!("Please, input your diagonal1 and diagonal2 for the equation.");
                for i in 0..2 {
                    let mut param = String::new();
                    io::stdin()
                        .read_line(&mut param)
                        .expect("failed to read input");
                    let param: f32 = param.trim().parse().expect("failed to parse input");
                    params[i] = param;
                }
                // assign the inputs
                let diagonal1: f32 = params[0];
                let diagonal2: f32 = params[1];
                // perform the calculations
                let area: f32 = rhom(diagonal1,diagonal2);
                println!("Your equation is 1/2 * ({diagonal1} + {diagonal2}) and the area of the rhombus is {area}cm^2");
            }
            // parallelogram area
            else if eqn == choices[2] {
                let mut params: [f32; 2] = [1.0, 2.0];
                println!("Please, input your base and altitude for the equation.");
                for i in 0..2 {
                    let mut param = String::new();
                    io::stdin()
                        .read_line(&mut param)
                        .expect("failed to read input");
                    let param: f32 = param.trim().parse().expect("failed to parse input");
                    params[i] = param;
                }
                // assign the inputs
                let base: f32 = params[0];
                let altitude: f32 = params[1];
                // perform the calculations
                let area: f32 = para(base,altitude);
                println!("Your equation is {base} * {altitude} and the area of the parallelogram is {area}cm^2");
            }
            // area of cube
            else if eqn == choices[3] {
                
                println!("Please, input your length of side for the equation.");
                
                    let mut param = String::new();
                    io::stdin()
                        .read_line(&mut param)
                        .expect("failed to read input");
                    let param: f32 = param.trim().parse().expect("failed to parse input");
                    
                // perform the calculations
                let area: f32 = cube(param);
                println!("Your equation is 6*({param})^2 and the area of the cube is {area}cm^2");
            }
            else if eqn == choices[4] {
                let mut params: [f32; 2] = [1.0, 2.0];
                println!("Please, input your radius and height for the equation.");
                for i in 0..2 {
                    let mut param = String::new();
                    io::stdin()
                        .read_line(&mut param)
                        .expect("failed to read input");
                    let param: f32 = param.trim().parse().expect("failed to parse input");
                    params[i] = param;
                }
                // assign the inputs
                let radius: f32 = params[0];
                let height: f32 = params[1];
                // perform the calculations
                let volume: f32 = vol(radius,height);
                println!("Your equation is {radius} * {height} and the area of the parallelogram is {volume}cm^3");
            }
        } 
        
        
        else {
            println!("Sorry, your choice is not a part of the list.");
        }
    i+=1;
    }
    println!("Thnak you!");
}

// FUNCTIONS

fn trap(height: f32, base1: f32, base2: f32) -> f32 {
    return (height/2.0) * (base1+base2);
}

fn rhom(d1:f32,d2:f32)->f32{
    return 0.5*(d1+d2);
}

fn para(ba:f32,al:f32)->f32{
    return ba*al;
}

fn cube(length:f32)->f32{
    return 6.0 * length.powf(2.0);
}

fn vol(rad:f32,height:f32)->f32{
    return (22.0/7.0)*rad.powf(2.0)*height;
}
