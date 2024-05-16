use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref T2: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("2", vec!["a", "b","c"]);
        map.insert("3", vec!["d", "e", "f"]);
        map.insert("4", vec!["g", "h", "i"]);
        map.insert("5", vec!["j", "k", "l"]);
        map.insert("6", vec!["m", "n", "o"]);
        map.insert("7", vec!["p", "q", "r","s"]);
        map.insert("8", vec!["t", "u", "v"]);
        map.insert("9", vec!["w", "x", "y", "z"]);
        map
    };
} 

fn get_letters(number: &str, letters: &str) -> String {
    let mut b = [0; 1];
    for i in number.chars() {
        if number.len() > 0 {
            let new_number = &number[1..];
            let i_string: &str = i.encode_utf8(&mut b);
            let new_letters = T2.get(&i_string);
            println!("New letters {:?}", new_letters);
            for j in new_letters.iter() {
                println!("litera {:?}", j);
                //let new_letters = letters.to_string() + &j.to_string() ;
                //get_letters(new_number, &new_letters);
            }
        }
    }
    return "alibaba".to_string();
}

fn main() {
    println!("Hello, world!");
    let letters = T2.get("2");
    println!("letters: {:?}", letters);
    get_letters("234", "");
}
