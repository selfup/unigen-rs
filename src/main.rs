extern crate rand;

use std::env;

mod builder;

fn main() {
    let mut size = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        size = args[1].clone();
    }

    let parsed_size = size.trim().parse::<u64>().unwrap();

    println!("Building Universe..");

    let mut universe = vec![];
    let mut neturon: [i16; 1] = [0];
    let mut proton: [i16; 1] = [0];
    let mut electron: [i16; 1] = [0];

    builder::initialize_universe(parsed_size, &mut universe);
    builder::particles(&mut universe, &mut neturon, &mut proton, &mut electron);

    println!("Universe built..\nChecking the charge..");

    builder::charge_of_field(&mut proton, &mut electron, parsed_size);
    builder::atom_charge(&mut universe);

    println!("Size of Universe: {:?}", universe.len());
}
