use std::io;

fn find_nemo(s: &str) -> String {
    let words: Vec<&str> = s.split(" ").collect();
    for (i, word) in words.iter().enumerate() {
        if *word == "Nemo" {
            return format!("I found Nemo at position {}!", i+1);
            
        }
    }
    return format!("I can't find Nemo :(");
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    println!("{}", find_nemo(&input));
}
