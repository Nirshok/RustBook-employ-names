use std::io;
use std::collections::HashMap;
fn main() {
    let mut workers: HashMap<String, Vec<String>> = HashMap::new();
    'outer: loop {

        println!("Input: \"Add *name* to *department*\"\n Press \"Enter\" to advance.\n");

        let mut add_name = String::new();

            io::stdin()
                .read_line(&mut add_name)
                .expect("Failed to read the line");
        
        let words: Vec<&str> = add_name.split_whitespace().collect();
        //println!("{:?}", &words);
        let lenght = words.len();
        //println!("{:?}", &lenght);

        if lenght == 4 {
                        
            let name = words[1];
            let depart = words[3];
            //handle the error with lenght less than 3
            workers.entry(depart.to_string())
                .or_insert_with(Vec::new)
                .push(name.to_string());
            //println!("{:?}", workers.get(depart));
            continue 'outer;

        } else if lenght < 4 && lenght > 0 || lenght > 4 {
            println!("Wrong input!\n");
            continue 'outer;
        }
            
        
        //println!("{:?}", workers);

        println!("Which department's names would you like to access?");


        let mut department = String::new();

            io::stdin()
                .read_line(&mut department)
                .expect("Failed to read the line");
        
        let departkey = department.trim().to_string();
        if workers.contains_key(&departkey) == true {

            let names = match workers.get_mut(&departkey) {
                Some(val) => val,
                None => {
                    println!("There is no one in this department.");
                    continue 'outer;
                }
            };
            
            names.sort();
            println!("People in {}are:{:#?}", departkey, names);
            break 'outer;

        } else {
            println!("This department is not written in!");
            continue 'outer;
        }                                           

    }

    
}

//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
//For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
//Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
