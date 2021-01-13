use std::env::args;

fn main() {
    let parsed_size = args().nth(1).unwrap().trim().parse::<u32>().unwrap();
    let _ = unigen::builder::generate_universe(parsed_size);
}
