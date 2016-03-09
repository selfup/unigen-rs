#[derive(Debug)]
struct LifeBlock {
    x_y:    (i32, i32),
    z:      Vec<i32>,
    charge: i32,
    mag:    i32,
}

fn main() {
    let mut universe = vec![];

    initialize_life(4, &mut universe);

    println!("{:?}", universe);
}

fn initialize_life(limit: i32, container: &mut Vec<LifeBlock>) {
    for v in 0..limit {
        for w in 0..limit{
            for q in 0..limit{
                container.push(LifeBlock { x_y: (v, w), z: vec![q], charge: 0, mag: 0 })
            }
        }
    }
}

#[test]
fn it_can_begin() {
    let mut universe = vec![];

    initialize_life(5, &mut universe);

    assert_eq!(universe[0].x_y, (0, 0));
    assert_eq!(universe[0].z, [0]);
    assert_eq!(universe[0].charge, 0);
    assert_eq!(universe[0].mag, 0);

    assert_eq!(universe[1].x_y, (0, 0));
    assert_eq!(universe[1].z, [1]);
    assert_eq!(universe[1].charge, 0);
    assert_eq!(universe[1].mag, 0);
}
