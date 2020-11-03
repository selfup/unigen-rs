use rand::Rng;
use rayon::prelude::*;

mod core;

pub fn initialize_universe(parsed_size: u64, uni: &mut Vec<core::LifeBlock>) -> Vec<core::LifeBlock> {
    for x in 0..parsed_size {
        for y in 0..parsed_size {
            for z in 0..parsed_size {                
                let (electrons, protons, neutrons): (i16, i16, i16) = (0, 0, 0);

                uni.push(core::LifeBlock {
                    x,
                    y,
                    z,
                    charge: 0,
                    atom: core::Atom {
                       electrons,
                        nucleus: core::Nucleus {
                            protons,
                            neutrons,
                        },
                    },
                })
            }
        }
    }

    // tradiontional lock is slow because of access serialization backpressure
    // let semaphoric_universe = Mutex::new(uni);
    // (0..(parsed_size * parsed_size * parsed_size)).into_par_iter().for_each_init(|| rand::thread_rng(), |rng, i| {

    let uni_copy = uni.clone();

    println!("Threads: {}", rayon::current_num_threads());

    let chunk_size = (parsed_size) as usize;

    uni.par_chunks_mut(chunk_size).for_each_init(|| rand::thread_rng(), |rng, blocks| {
        let (electrons, protons, neutrons): (i16, i16, i16) = (
            rng.gen_range(0, 118),
            rng.gen_range(0, 118),
            rng.gen_range(0, 118),
        );

        for block in blocks {
            block.atom.electrons = electrons;
            block.atom.nucleus.protons = protons;
            block.atom.nucleus.neutrons = neutrons;
        }
    });

    uni_copy
}

pub fn charge_of_field(proton: &mut [i16; 1], electron: &mut [i16; 1], u: u64) {
    let size = u * u * u;
    let true_size = size as i16;
    
    if proton[0] == true_size && electron[0] == true_size {
        println!("Field is Netural");
    } else if (proton[0] > true_size) && (electron[0] < proton[0]) {
        println!("Field is Ionic");
    } else {
        println!("Field is Anionic");
    }
}

pub fn atom_charge(universe: &mut Vec<core::LifeBlock>) {
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
fn it_can_begin() {
    let mut universe = vec![];
    
    initialize_universe(5, &mut universe);

    assert_eq!(universe.len(), 125);
    assert_eq!(universe[0].x_y, (0, 0));
    assert_eq!(universe[0].z, 0);
    assert_eq!(universe[20].x_y, (0, 4));
    assert_eq!(universe[20].z, 0);
}

pub fn particles(universe: &mut Vec<core::LifeBlock>, neutron: &mut [i16; 1], proton: &mut [i16; 1], electron: &mut [i16; 1]) {
    neutron[0] = universe.par_iter().map(|i| i.atom.nucleus.neutrons).sum();
    proton[0] = universe.par_iter().map(|i| i.atom.nucleus.protons).sum();
    electron[0] = universe.par_iter().map(|i| i.atom.electrons).sum();
}

#[test]
fn it_can_infer_the_charge_of_an_atom() {
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
