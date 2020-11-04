extern crate rand;

use std::env;

mod builder;

fn main() {
    let mut size = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        size = args[1].clone();
    }

    let parsed_size = size.trim().parse::<u32>().unwrap();

    println!("Building Universe..");

    let mut universe = vec![];
    let mut neturon: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    let mut generated_universe = builder::initialize_universe(parsed_size, &mut universe);
    
    builder::particles(&mut generated_universe, &mut neturon, &mut proton, &mut electron);

    println!("Snapshot..\n\n{:?}\n", &generated_universe[0]);
    println!("Universe built!\nChecking the charge..");

    builder::charge_of_field(&mut proton, &mut electron, parsed_size);
    builder::atom_charge(&mut generated_universe);

    println!("Size of Universe: {:?}", generated_universe.len());
}
