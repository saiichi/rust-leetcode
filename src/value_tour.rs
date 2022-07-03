use std::collections::HashMap;
use std::thread;

pub fn value_tour() {
    let authenticated = true;
    // if authenticated {
    //     todo!()
    // } else {
    //     todo!()
    // }

    let mut total = 0;
    total += 1;

    let name = "John".to_string();
    print_my_name(name);

    let hash_map: HashMap<String, String> = HashMap::new();
    print_map(&hash_map);
    
    let mut data = vec![1, 2, 3];
    thread::spawn(move || {
        data.push(4);
    });
}

fn print_my_name(name: String) {
    println!("{}", name);
}

fn print_map(map: &HashMap<String, String>) {
    for (key, value) in map {
        println!("{} {}", key, value);
    }
}