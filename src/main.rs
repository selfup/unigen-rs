use std::io;

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

    initialize_life(trimmed, &mut universe);

    println!("{:?}", universe);
}

fn initialize_life(limit: i32, container: &mut Vec<LifeBlock>) {
    for v in 0..limit + 1 {
        for w in 0..limit + 1 {
            for q in 0..limit + 1 {
                container.push(LifeBlock { x_y: (v, w), z: q,
                               charge: atom::Atom { electrons: 1,
                                                    nucleus: atom::Nucleus {protons: 1, neutrons: 1}
                                                  }
                                         }
                              )
            }
        }
    }
}

#[test]
fn it_can_begin() {
    let mut universe = vec![];

    initialize_life(5, &mut universe);

    assert_eq!(universe.len(), 216);

    assert_eq!(universe[0].x_y, (0, 0));
    assert_eq!(universe[0].z, 0);

    assert_eq!(universe[1].x_y, (0, 0));
    assert_eq!(universe[1].z, 1);

    assert_eq!(universe[2].x_y, (0, 0));
    assert_eq!(universe[2].z, 2);

    assert_eq!(universe[6].x_y, (0, 1));
    assert_eq!(universe[6].z, 0);

    assert_eq!(universe[7].x_y, (0, 1));
    assert_eq!(universe[7].z, 1);

    assert_eq!(universe[8].x_y, (0, 1));
    assert_eq!(universe[8].z, 2);

    assert_eq!(universe[12].x_y, (0, 2));
    assert_eq!(universe[12].z, 0);

    assert_eq!(universe[13].x_y, (0, 2));
    assert_eq!(universe[13].z, 1);

    assert_eq!(universe[20].x_y, (0, 3));
    assert_eq!(universe[20].z, 2);
}
