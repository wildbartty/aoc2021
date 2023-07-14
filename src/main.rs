use std::{env, string};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args); 
    println!("Hello, world!");
}
