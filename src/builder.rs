use colored::*;
use rand::Rng;
use rayon::prelude::*;

pub mod core;

use self::core::{Atom, Baryon, Block, Nucleus, neutron::Neutrons, proton::Protons};

const CHUNK_SIZE: usize = 128;

pub struct Blocks {}

#[inline]
fn index_to_xyz(parsed_size: u32, idx: u32) -> (u32, u32, u32) {
    let sz_2 = parsed_size * parsed_size;
    let x = idx / sz_2;
    let w = idx % sz_2;
    let y = w / parsed_size;
    let z = w % parsed_size;
    (x, y, z)
}

pub enum Charge {
    Neutral,
    Anionic,
    Cationic,
}

impl Blocks {
    pub fn initialize_universe(parsed_size: u32) -> Vec<Block> {
        let total_size: u32 = parsed_size
            .checked_mul(parsed_size.checked_mul(parsed_size).unwrap())
            .unwrap();

        let vec_size: usize = total_size as usize;
        let mut container: Vec<Block> = Vec::with_capacity(vec_size);

        container.par_extend((0..total_size).into_par_iter().map(|i| {
            let (x, y, z) = index_to_xyz(parsed_size, i);
            let (electrons, protons, neutrons): (u8, u8, u8) = (0, 0, 0);

            let generated_protons = Protons::new(protons);
            let generated_neutrons = Neutrons::new(neutrons);

            Block {
                id: i,
                x: x as i32,
                y: y as i32,
                z: z as i32,
                charge: 0,
                atom: Atom {
                    electrons,
                    nucleus: Nucleus {
                        baryon: Baryon {
                            protons: generated_protons,
                            neutrons: generated_neutrons,
                        },
                    },
                },
            }
        }));

        container
    }

    pub fn particles(
        universe: &mut [Block],
        neutron: &mut [u32; 1],
        proton: &mut [u32; 1],
        electron: &mut [u32; 1],
    ) {
        rayon::scope(|s| {
            s.spawn(|_| {
                neutron[0] = universe
                    .par_iter()
                    .map(|i| i.atom.nucleus.baryon.neutrons.count as u32)
                    .sum();
            });

            s.spawn(|_| {
                proton[0] = universe
                    .par_iter()
                    .map(|i| i.atom.nucleus.baryon.protons.count as u32)
                    .sum();
            });

            s.spawn(|_| {
                electron[0] = universe.par_iter().map(|i| i.atom.electrons as u32).sum();
            });
        });
    }

    pub fn charge_of_field(proton: &mut [u32; 1], electron: &mut [u32; 1], u: u32) -> Charge {
        let size: u32 = u * u * u;

        if proton[0] == size && electron[0] == size {
            Charge::Neutral
        } else if (proton[0] > size) && (electron[0] < proton[0]) {
            Charge::Cationic
        } else {
            Charge::Anionic
        }
    }

    pub fn atom_charge(universe: &mut [Block]) {
        universe.par_chunks_mut(CHUNK_SIZE).for_each(|chunk| {
            for block in chunk {
                calculate_charge(block)
            }
        });
    }

    pub fn tick(universe: &mut [Block]) {
        universe
            .par_chunks_mut(CHUNK_SIZE)
            .for_each_init(rand::rng, |rng, chunk| {
                for block in chunk {
                    mutate_blocks_with_new_particles(rng, block);
                }
            });
    }
}

#[inline]
pub fn calculate_charge(block: &mut Block) {
    use std::cmp::Ordering::*;

    block.charge = match u8::cmp(
        &block.atom.nucleus.baryon.protons.count,
        &block.atom.electrons,
    ) {
        Equal => 0,
        Greater => 1,
        Less => -1,
    }
}

#[inline]
pub fn mutate_blocks_with_new_particles<R: Rng>(rng: &mut R, block: &mut Block) {
    let (electrons, protons, neutrons, rotation): (u8, u8, u8, u8) = (
        rng.random_range(0..118),
        rng.random_range(0..118),
        rng.random_range(0..118),
        rng.random_range(1..7),
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
    block.atom.nucleus.baryon.protons = Protons::new(protons);
    block.atom.nucleus.baryon.neutrons = Neutrons::new(neutrons);
}

pub fn generate_universe(parsed_size: u32) -> Vec<Block> {
    let separator: &str = "--------------------------------";

    println!("{}", &separator.magenta().bold());

    let mut neutron: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    println!("Threads: {}\nBuilding..", rayon::current_num_threads());

    let mut generated_universe: Vec<Block> = Blocks::initialize_universe(parsed_size);

    Blocks::tick(&mut generated_universe);
    Blocks::particles(
        &mut generated_universe,
        &mut neutron,
        &mut proton,
        &mut electron,
    );

    println!("{}", &separator.yellow().bold());

    println!("Universe built");

    println!("{}", &separator.green().bold());

    println!("Calculating charge of field..");

    println!("{}", &separator.yellow().bold());

    let charge: Charge = Blocks::charge_of_field(&mut proton, &mut electron, parsed_size);

    match charge {
        Charge::Neutral => println!("Field is Netural"),
        Charge::Cationic => println!("Field is Cationic"),
        Charge::Anionic => println!("Field is Anionic"),
    }

    Blocks::atom_charge(&mut generated_universe);

    println!("{}", &separator.green().bold());

    let default_baryons: usize = 236;
    let quarks_per_baryon: usize = 3;
    let generated_universe_length: usize = generated_universe.len();

    let total_atoms: usize = generated_universe_length;
    let total_baryons: usize = generated_universe_length * default_baryons;
    let total_quarks: usize = generated_universe_length * default_baryons * quarks_per_baryon;

    println!("Atoms: {}", pretty_print_nums(&total_atoms));
    println!("Baryons: {}", pretty_print_nums(&total_baryons));
    println!("Quarks: {}", pretty_print_nums(&total_quarks));

    println!("{}", &separator.magenta().bold());

    let total_objects: usize = &total_atoms + &total_baryons + &total_quarks;

    println!(
        "Total high-level objects in memory: {}",
        pretty_print_nums(&total_objects)
    );

    println!("{}", &separator.green().bold());

    generated_universe
}

fn pretty_print_nums(num: &usize) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

#[test]
fn it_can_pretty_print_large_numbers() {
    let nums: Vec<(usize, String)> = vec![
        (4, String::from("4")),
        (40, String::from("40")),
        (400, String::from("400")),
        (4_000, String::from("4,000")),
        (40_000, String::from("40,000")),
        (400_000, String::from("400,000")),
        (4_000_000, String::from("4,000,000")),
        (40_000_000, String::from("40,000,000")),
        (400_000_000, String::from("400,000,000")),
        (4_000_000_000, String::from("4,000,000,000")),
        (40_000_000_000, String::from("40,000,000,000")),
        (400_000_000_000, String::from("400,000,000,000")),
        (4_000_000_000_000, String::from("4,000,000,000,000")),
    ];

    for num in nums {
        assert_eq!(pretty_print_nums(&num.0), num.1)
    }
}

#[test]
fn it_can_begin() {
    let universe: Vec<Block> = Blocks::initialize_universe(5);

    assert_eq!(universe.len(), 125);

    assert_eq!(universe[0].x, 0);
    assert_eq!(universe[0].y, 0);
    assert_eq!(universe[0].z, 0);

    assert_eq!(universe[20].x, 0);
    assert_eq!(universe[20].y, 4);
    assert_eq!(universe[20].z, 0);
}

#[test]
fn it_can_infer_the_charge_of_an_atom() {
    let mut neutron: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    let mut generated_universe: Vec<Block> = Blocks::initialize_universe(5);

    Blocks::tick(&mut generated_universe);

    Blocks::particles(
        &mut generated_universe,
        &mut neutron,
        &mut proton,
        &mut electron,
    );

    Blocks::atom_charge(&mut generated_universe);

    assert_eq!(generated_universe.len(), 125);
}

#[test]
fn it_can_sense_the_field() {
    let mut neutron: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    let mut universe: Vec<Block> = Blocks::initialize_universe(2);

    Blocks::particles(&mut universe, &mut neutron, &mut proton, &mut electron);

    assert_eq!(universe.len(), 8);
    assert_eq!(neutron.len(), 1);
    assert_eq!(proton.len(), 1);
    assert_eq!(electron.len(), 1);
}

#[test]
fn it_can_calculate_xyz_from_index() {
    let size = 5;

    assert_eq!(index_to_xyz(size, 0), (0, 0, 0));
    assert_eq!(index_to_xyz(size, 1), (0, 0, 1));
    assert_eq!(index_to_xyz(size, 5), (0, 1, 0));
    assert_eq!(index_to_xyz(size, 25), (1, 0, 0));
    assert_eq!(index_to_xyz(size, 124), (4, 4, 4));
}
