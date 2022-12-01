use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn slurp_file(filepath: &str) -> String {    
    let path = Path::new(filepath);
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open file {}: {}", display, why),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(s) => s
    };

    return s;
}
