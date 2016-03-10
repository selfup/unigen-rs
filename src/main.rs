extern crate rayon;
extern crate rand;

use std::io;
use rand::Rng;

mod atom;

#[derive(Debug)]
struct LifeBlock {
    x_y:    (i32, i32),
    z:      i32,
    charge: atom::Atom,
}

fn main() {
    println!("Size of universe. Please:");
    let mut size = String::new();

    io::stdin().read_line(&mut size).expect("Failed to read line");
    let trimmed = size.trim().parse::<i32>().unwrap();

    let mut universe = vec![];
    let mut neut = vec![0];
    let mut prot = vec![0];
    let mut elec = vec![0];

    initialize_life(trimmed, &mut universe);
    particles(&mut universe, &mut neut, &mut prot, &mut elec);
    is_field_neutral(&mut neut, &mut prot, &mut elec, trimmed);
    charge_of_field(&mut prot, &mut elec, trimmed);

    println!("Size of Universe: {:?}", universe.len());
}

fn initialize_life(limit: i32, uni: &mut Vec<LifeBlock>) {

    for v in 0..limit + 1 {
        for w in 0..limit + 1 {
            for q in 0..limit + 1 {
                let n1: i32 = rand::thread_rng().gen_range(0, 118);
                let n2: i32 = rand::thread_rng().gen_range(0, 118);
                let n3: i32 = rand::thread_rng().gen_range(0, 118);
                uni.push(LifeBlock { x_y: (v, w), z: q,
                           charge: atom::Atom { electrons: n1,
                                                nucleus: atom::Nucleus {protons: n2, neutrons: n3}
                                              }
                                     }
                          )
            }
        }
    }
}

use rayon::prelude::*;
fn particles(input: &mut Vec<LifeBlock>, n: &mut Vec<i32>, p: &mut Vec<i32>, e: &mut Vec<i32>) {
    n[0] = input.par_iter().map(|i| i.charge.nucleus.neutrons).sum();
    p[0] = input.par_iter().map(|i| i.charge.nucleus.protons).sum();
    e[0] = input.par_iter().map(|i| i.charge.electrons).sum();
}

fn is_field_neutral(n: &mut Vec<i32>, p: &mut Vec<i32>, e: &mut Vec<i32>, u: i32) {
    let size = (u + 1) * (u + 1) * (u + 1);
    if n[0] == size && p[0] == size && e[0] == size {
        println!("NEUTRAL");
    } else {
        println!("NOT NEUTRAL");
    }
}

fn charge_of_field(p: &mut  Vec<i32>, e: &mut Vec<i32>, u: i32) {
    let size = (u + 1) * (u + 1) * (u + 1);

    if p[0] == size && e[0] == size {
        println!("field has a neutral charge");
    } else if p[0] > size && e[0] < p[0] {
        println!("field is ionic");
    } else {
        println!("field is anionic");
    }
}

#[test]
fn it_can_begin() {
    let mut universe = vec![];

    initialize_life(5, &mut universe);

    assert_eq!(universe.len(), 216);

    assert_eq!(universe[0].x_y, (0, 0));
    assert_eq!(universe[0].z, 0);

    assert_eq!(universe[20].x_y, (0, 3));
    assert_eq!(universe[20].z, 2);
}
