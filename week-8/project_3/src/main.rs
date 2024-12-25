use std::fs::File;
use std::io::Write;

fn main(){
    // Arrays for the datasets
    let head = ["CONVICTED MINISTERS"];
    let headers = ["S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE"];
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];
    let sn = [1,2,3,4,5];
  

    // Writing the data to a text file
    let mut file = File::create("/Users/chiomadande/Documents/c.adandeCSC101/week-8/project_3/target/efcc.txt").expect("Could not create file");
    writeln!(file,"\n{:^75}\n", head[0]).expect("failed to add input");
    writeln!(file,"{:<5}{:<30}{:<20}{:<20}", headers[0], headers[1], headers[2], headers[3]).expect("failed to add input");
    writeln!(file,"{:-<75}\n\n", "").expect("failed to add input");
    for i in 0..ministries.len(){
        writeln!(file,"{:<5}{:<30}{:<20}{:<20}\n",sn[i], commissioners[i], ministries[i], geopolitical_zones[i]).expect("failed to add input");
    }

    println!("\nFINISHED! The efcc file can be found in the target folder of this project. Thank you!");
    
}
