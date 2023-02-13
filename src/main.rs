use std::io;
use std::collections::HashMap;
use employ_names::*;

fn main() {
    let mut workers: HashMap<String, Vec<String>> = HashMap::new();
    'outer: loop {

        println!("Input: \"Add *name* to *department*\"\n(Input space or nothing to advance.)\n");

        let mut add_name = String::new();

            io::stdin()
                .read_line(&mut add_name)
                .expect("Failed to read the line");

        let words = into_str_vec(&add_name);
        let lenght = words.len();

        if lenght == 4 {                        
            insert_k_v(&mut workers, words);
            continue 'outer;

        } else if lenght < 4 && lenght > 0 || lenght > 4 {
            println!("Wrong input!\n");
            continue 'outer;
        }

        remove_copies(&mut workers);  

        loop {
        println!("Which department's names would you like to access?\n(Input space or nothing to finish this session.)");

        let mut department = String::new();
        
            io::stdin()
                .read_line(&mut department)
                .expect("Failed to read the line");

        match map_output(department, &mut workers) {
            0 | 2 => continue 'outer,
            1 => continue,
            3 => break,
            _ => {
                println!("Unknown error");
                break;
            },
        }

        
    }
    break;
    }

    
}
