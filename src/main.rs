use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let query = &args[0];
    let filename = &args[1];
    println!("searching for {}", query);
    println!("in file {}", filename);
}
