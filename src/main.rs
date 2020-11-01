extern crate rand;

use rand::Rng;
use std::env;

mod atom;

#[derive(Debug)]
struct LifeBlock {
    x_y: (u64, u64),
    z: u64,
    charge: i16,
    atom: atom::Atom,
}

fn main() {
    let mut size = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        size = args[1].clone();
    }

    let parsed_size = size.trim().parse::<u64>().unwrap();

    let mut universe = vec![];
    let mut neturon: [i16; 1] = [0];
    let mut proton: [i16; 1] = [0];
    let mut electron: [i16; 1] = [0];

    initialize_universe(parsed_size, &mut universe);
    particles(&mut universe, &mut neturon, &mut proton, &mut electron);
    charge_of_field(&mut proton, &mut electron, parsed_size);
    atom_charge(&mut universe);

    println!("Size of Universe: {:?}", universe.len());
}

fn initialize_universe(parsed_size: u64, uni: &mut Vec<LifeBlock>) {
    let mut rng = rand::thread_rng();

    for x in 0..parsed_size {
        for y in 0..parsed_size {
            for z in 0..parsed_size {
                let (n1, n2, n3): (i16, i16, i16) = (
                    rng.gen_range(0, 118),
                    rng.gen_range(0, 118),
                    rng.gen_range(0, 118),
                );

                uni.push(LifeBlock {
                    x_y: (x, y),
                    z,
                    charge: 0,
                    atom: atom::Atom {
                        electrons: n1,
                        nucleus: atom::Nucleus {
                            protons: n2,
                            neutrons: n3,
                        },
                    },
                })
            }
        }
    }
}

#[test]
fn it_can_begin() {
    let mut universe = vec![];
    
    initialize_universe(5, &mut universe);

    assert_eq!(universe.len(), 125);
    assert_eq!(universe[0].x_y, (0, 0));
    assert_eq!(universe[0].z, 0);
    assert_eq!(universe[20].x_y, (0, 4));
    assert_eq!(universe[20].z, 0);
}

fn particles(universe: &mut Vec<LifeBlock>, neutron: &mut [i16; 1], proton: &mut [i16; 1], electron: &mut [i16; 1]) {
    neutron[0] = universe.iter().map(|i| i.atom.nucleus.neutrons).sum();
    proton[0] = universe.iter().map(|i| i.atom.nucleus.protons).sum();
    electron[0] = universe.iter().map(|i| i.atom.electrons).sum();
}

#[test]
fn it_can_sense_the_field() {
    let mut universe = vec![];

    let mut neturon: [i16; 1] = [0];
    let mut proton: [i16; 1] = [0];
    let mut electron: [i16; 1] = [0];

    initialize_universe(2, &mut universe);
    particles(&mut universe, &mut neturon, &mut proton, &mut electron);

    assert_eq!(universe.len(), 8);
    assert_eq!(neturon.len(), 1);
    assert_eq!(proton.len(), 1);
    assert_eq!(electron.len(), 1);
}

fn charge_of_field(proton: &mut [i16; 1], electron: &mut [i16; 1], u: u64) {
    let size = u * u * u;
    let true_size = size as i16;
    
    if proton[0] == true_size && electron[0] == true_size {
        println!("field has a neutral charge");
    } else if (proton[0] > true_size) && (electron[0] < proton[0]) {
        println!("field is ionic");
    } else {
        println!("field is anionic");
    }
}

fn atom_charge(universe: &mut Vec<LifeBlock>) {
    for block in universe {
        if block.atom.nucleus.protons == block.atom.electrons {
            block.charge = 0;
        } else if block.atom.nucleus.protons > block.atom.electrons {
            block.charge = 1;
        } else {
            block.charge = -1;
        }
    }
}

#[test]
fn it_can_dictate_an_atoms_charge() {
    let mut universe = vec![];

    let mut neturon: [i16; 1] = [0];
    let mut proton: [i16; 1] = [0];
    let mut electron: [i16; 1] = [0];
    let mut rand_nums = vec![0];
    
    let mut rando = "";

    initialize_universe(5, &mut universe);
    particles(&mut universe, &mut neturon, &mut proton, &mut electron);
    atom_charge(&mut universe);

    assert_eq!(universe.len(), 125);

    for u in universe {
        rand_nums.push(u.charge)
    }

    rand_nums.sort();
    rand_nums.dedup();

    if rand_nums.len() > 1 {
        rando = "random"
    }

    assert_eq!(rando, "random");
}
