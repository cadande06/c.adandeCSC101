

use std::fs::OpenOptions;
use std::io::{Write};

fn main(){
    // creating file
    let mut file = OpenOptions::new().append(true).create(true).open("/Users/chiomadande/Documents/c.adandeCSC101/week-8/project_1/target/table.txt").expect("cannot open file");
    file.write_all("\n+++++++++++++      NIGERIAN BREWERIES PLC      +++++++++++++\n".as_bytes()).expect("write failed");
    
    // categories
    let cat = ["LAGER", "STOUT", "NON-ALCOHOLIC"];
    
    //group of drinks
    let drinks = [
        ["33 Export", "Legend", "Maltina"],
        ["Desperados", "Turbo King", "Amstel Malta"],
        ["Goldberg", "Williams", "Malta Gold"],
        ["Gulder", "", "Fayrouz"],
        ["Heineken", "", ""],
        ["Star","",""]
    ];
    
    // adding headers
    writeln!(file,"\n{:<20}{:<20}{:<20}",cat[0],cat[1],cat[2]).expect("failed to write headers");

    // adding body
    for drink in drinks{
        writeln!(file,"\n{:<20}{:<20}{:<20}",drink[0],drink[1],drink[2]).expect("failed to write drink data");
    }
    
    println!("\nTABLE COMPLETE! Check the target folder of this project for the table txt file.\n");
}