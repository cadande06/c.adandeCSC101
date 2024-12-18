use std::fs::{self, File};
use std::io::{self, Read, Write};
fn main() {
    // creating the three separate data files
    // file 1
    let mut headn=["S/N", "NAME OF COMMISIONER"];
    let mut name_list= [
    ["1","Aigbogun Alamba Daudu"], 
    ["2","Murtala Afeez Bendu"],
    ["3", "Okorocha Calistus Ogbona"],
    ["4", "Adewale Jimoh Akanbi"],
    ["5", "Osazuwa Faith Etieye"]
    ];
    let mut names = File::create("/Users/chiomadande/Documents/c.adandeCSC101/week-8/project_2/target/names.txt").expect("failed to create file");
    writeln!(names,"{:<5}{:<30}", headn[0], headn[1]).unwrap();
    writeln!(names,"{:-<35}\n", "").unwrap();
    for n in name_list{
    writeln!(names,"{:<5}{:<30}", n[0], n[1]).unwrap();
    writeln!(names,"{:-<35}\n", "").unwrap();
    }

    // file 2
    let mut headm=["S/N", "MINISTRY"];
    let mut min_list= [
    ["1","Internal Affairs"], 
    ["2","Justice"],
    ["3", "Defense"],
    ["4", "Power & Steel"],
    ["5", "Petroleum"]
    ];
    let mut ministries = File::create("/Users/chiomadande/Documents/c.adandeCSC101/week-8/project_2/target/ministries.txt").expect("failed to create file");
    writeln!(ministries,"{:<5}{:<20}", headm[0], headm[1]).unwrap();
    writeln!(ministries,"{:-<25}\n", "").unwrap();
    for m in min_list{
    writeln!(ministries,"{:<5}{:<20}", m[0], m[1]).unwrap();
    writeln!(ministries,"{:-<25}\n", "").unwrap();
    }

    // file 3
    let mut headg=["S/N", "GEOPOLITICAL ZONE"];
    let mut geo_list= [
    ["1","South West"], 
    ["2","North East"],
    ["3", "South South"],
    ["4", "South West"],
    ["5", "South East"]
    ];
    let mut geos = File::create("/Users/chiomadande/Documents/c.adandeCSC101/week-8/project_2/target/geos.txt").expect("failed to create file");
    writeln!(geos,"{:<5}{:<25}", headg[0], headg[1]).unwrap();
    writeln!(geos,"{:-<30}\n", "").unwrap();
    for g in geo_list{
    writeln!(geos,"{:<5}{:<25}", g[0], g[1]).unwrap();
    writeln!(geos,"{:-<30}\n", "").unwrap();
    }
}
