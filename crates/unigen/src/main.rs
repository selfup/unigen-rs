use std::env::args;

mod builder;

fn main() {
    let mut size = String::new();
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        size = args[1].clone();
    }

    let parsed_size = size.trim().parse::<u32>().unwrap();

    let _ = builder::generate_universe(parsed_size);
}
