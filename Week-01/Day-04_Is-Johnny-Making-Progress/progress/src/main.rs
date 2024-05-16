fn progress(data_in: Vec<u32>) -> u32 {
    let mut data_out = 0;
    if data_in.len() < 2 {
        return 0;
    }
    for i in 1..data_in.len() {
        if data_in[i] > data_in[i - 1] {
            data_out += 1;
        }
    }
    return data_out;
}

fn main() {
    println!("Hello, world!");
    println!("{}", progress(vec![9, 9]));
}
