use rand::Rng;
use colored::*;
use rayon::prelude::*;

pub mod core;

use self::core::{proton, neutron};

pub struct Blocks {}

fn index_to_xyz(size: u32 , idx: u32) -> (u32 , u32 , u32){
    let sz_2 = size * size;
    let x = idx / sz_2;
    let w = idx % sz_2;
    let y = w / size;
    let z = w % size;
    (x , y ,z)
}

impl Blocks {
    pub fn initialize_universe(parsed_size: u32) -> Vec<core::Block> {
        let mut id: u32 = 0;
        let total_size = parsed_size.checked_mul(parsed_size.checked_mul(parsed_size).unwrap()).unwrap();
        
        let vec_size = total_size as usize;
        
        
        println!("Threads: {}\nBuilding..", rayon::current_num_threads());


        (0..total_size).into_par_iter().map(|i|{
            let (x , y , z) = index_to_xyz(parsed_size , i);
            let (electrons, protons, neutrons): (u32, u32, u32) = (0, 0, 0);

                    let generated_protons = proton::Protons::new(protons);
                    let generated_neutrons = neutron::Neutrons::new(neutrons);
    
            core::Block {
                        id,
                        x,
                        y,
                        z,
                        charge: 0,
                        atom: core::Atom {
                           electrons,
                            nucleus: core::Nucleus {
                                baryon: core::Baryon {
                                    protons: generated_protons,
                                    neutrons: generated_neutrons,
                                },
                            },
                        },
                    }
        }).collect()
        
        /*
        for x in 0..parsed_size {
            for y in 0..parsed_size {
                for z in 0..parsed_size {                
                    let (electrons, protons, neutrons): (u32, u32, u32) = (0, 0, 0);

                    let generated_protons = proton::Protons::new(protons);
                    let generated_neutrons = neutron::Neutrons::new(neutrons);
    
                    universe.push(core::Block {
                        id,
                        x,
                        y,
                        z,
                        charge: 0,
                        atom: core::Atom {
                           electrons,
                            nucleus: core::Nucleus {
                                baryon: core::Baryon {
                                    protons: generated_protons,
                                    neutrons: generated_neutrons,
                                },
                            },
                        },
                    });
    
                    id += 1;
                }
            }
        }
    
        universe
        */
    }
    
    pub fn particles(universe: &mut [core::Block], neutron: &mut [u32; 1], proton: &mut [u32; 1], electron: &mut [u32; 1]) {
        rayon::scope(|s| {
            s.spawn(|_| {
                neutron[0] = universe.par_iter().map(|i| i.atom.nucleus.baryon.neutrons.count).sum();
            });
            s.spawn(|_|{
                proton[0] = universe.par_iter().map(|i| i.atom.nucleus.baryon.protons.count).sum();
            });
            s.spawn(|_|{
                electron[0] = universe.par_iter().map(|i| i.atom.electrons).sum();
            });
        });
    }
    
    pub fn charge_of_field(proton: &mut [u32; 1], electron: &mut [u32; 1], u: u32) {
        let size: u32 = u * u * u;
        let cast_size: u32 = size as u32;
        
        if proton[0] == cast_size && electron[0] == cast_size {
            println!("Field is Netural");
        } else if (proton[0] > cast_size) && (electron[0] < proton[0]) {
            println!("Field is Cationic");
        } else {
            println!("Field is Anionic");
        }
    }
    
    pub fn atom_charge(universe: &mut [core::Block]) {
        for block in universe {
            calculate_charge(block);
        }
    }
    
    pub fn tick(universe: &mut [core::Block]) {

        universe.par_iter_mut().for_each_init(rand::thread_rng , |rng, block| {
                mutate_blocks_with_new_particles(rng, block);
        });
    }
}

pub fn calculate_charge(block: &mut core::Block) {
    if block.atom.nucleus.baryon.protons.count == block.atom.electrons {
        block.charge = 0;
    } else if block.atom.nucleus.baryon.protons.count > block.atom.electrons {
        block.charge = 1;
    } else {
        block.charge = -1;
    }
}

pub fn mutate_blocks_with_new_particles<R: Rng>(rng: &mut R, block: &mut core::Block) {
    let (electrons, protons, neutrons, rotation): (u32, u32, u32, u8) = (
        rng.gen_range(0..118),
        rng.gen_range(0..118),
        rng.gen_range(0..118),
        rng.gen_range(1..6),
    );

    match rotation {
        1 => block.x += 1,
        2 => block.x -= 1,
        3 => block.y += 1,
        4 => block.y -= 1,
        5 => block.z += 1,
        6 => block.z -= 1,
        _ => (),
    }

    block.atom.electrons = electrons;
    block.atom.nucleus.baryon.protons = proton::Protons::new(protons);
    block.atom.nucleus.baryon.neutrons = neutron::Neutrons::new(neutrons);
}

pub fn generate_universe(parsed_size: u32) -> Vec<core::Block> {
    println!("{}", "--------------------------------".red().bold());

    let mut neturon: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    let mut generated_universe = Blocks::initialize_universe(parsed_size);

    Blocks::tick(&mut generated_universe);
    Blocks::particles(&mut generated_universe, &mut neturon, &mut proton, &mut electron);

    println!("{}", "--------------------------------".purple().bold());
    println!("Universe built!");
    println!("{}", "--------------------------------".yellow().bold());

    Blocks::charge_of_field(&mut proton, &mut electron, parsed_size as u32);
    Blocks::atom_charge(&mut generated_universe);

    let default_baryons = 236;
    let quarks_per_baryon = 3;
    let generated_universe_length = generated_universe.len();
    let total_atoms = generated_universe_length;
    let total_baryons = generated_universe_length * default_baryons;
    let total_quarks = generated_universe_length * default_baryons* quarks_per_baryon;

    println!("{}", "--------------------------------".magenta().bold());
    println!("Atoms: {}", total_atoms);
    println!("Baryons: {}", total_baryons);
    println!("Quarks: {}", total_quarks);
    println!("{}", "--------------------------------".red().bold());

    generated_universe
}

#[test]
fn it_can_begin() {    
    let universe = Blocks::initialize_universe(5);

    assert_eq!(universe.len(), 125);
    
    assert_eq!(universe[0].x,0);
    assert_eq!(universe[0].y,0);
    assert_eq!(universe[0].z,0);

    assert_eq!(universe[20].x,0);
    assert_eq!(universe[20].y, 4);
    assert_eq!(universe[20].z,0);
}

#[test]
fn it_can_infer_the_charge_of_an_atom() {
    let mut neturon: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];
    
    let mut generated_universe: Vec<core::Block> = Blocks::initialize_universe(5);
    Blocks::tick(&mut generated_universe);
    Blocks::particles(&mut generated_universe, &mut neturon, &mut proton, &mut electron);
    Blocks::atom_charge(&mut generated_universe);
    
    assert_eq!(generated_universe.len(), 125);
}

#[test]
fn it_can_sense_the_field() {
    let mut neturon: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    let mut universe = Blocks::initialize_universe(2);
    Blocks::particles(&mut universe, &mut neturon, &mut proton, &mut electron);

    assert_eq!(universe.len(), 8);
    assert_eq!(neturon.len(), 1);
    assert_eq!(proton.len(), 1);
    assert_eq!(electron.len(), 1);
}
