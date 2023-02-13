use std::collections::HashMap;

pub fn into_str_vec<'a>(s: &'a str) -> Vec<&'a str> {
    s.split_whitespace().collect()
}

pub fn insert_k_v(workers: &mut HashMap<String, Vec<String>>, words: Vec<&str>) {
    let name = words[1];
    let depart = words[3];
    
    workers.entry(depart.to_string())
        .or_insert_with(Vec::new)
        .push(name.to_string());
}

pub fn remove_copies(hashmap: &mut HashMap<String, Vec<String>>) {
    for val in hashmap.values_mut() {
        val.sort_unstable();
        val.dedup();
    }
}

pub fn map_output(key: String, map: &mut HashMap<String, Vec<String>>) -> u8 {
    let vect: Vec<&str> = into_str_vec(&key);
    let departkey = key.trim().to_string();

    if map.contains_key(&departkey) {
        let names = match map.get_mut(&departkey) {
            Some(val) => val,
            None => {
                println!("There is no one in this department.");
                return 0;
            }
        };
        
        names.sort();

        println!("People in {} are:{:#?}", departkey, names);
        return 1;
    } else if vect.len() > 0 {
        println!("This department is not written in!");
        return 2;
    } else {
        println!("Shutting down...");
        return 3;
    }
}

#[cfg(test)]


mod test {
    use std::collections::HashMap;
    use super::*;
    #[test]
    fn works() {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let words = vec!["KEK", "my", "LUL", "VVV"];

        insert_k_v(&mut map, words);

        let string_vec: Vec<String> = vec!["my".to_string()];
        let stringus = "VVV".to_string();

        let mut second_map: HashMap<String, Vec<String>> = HashMap::new();
        second_map.insert(stringus, string_vec);

        assert_eq!(map, second_map)

    }
}