struct LifeBlock {
    x_y: (i32, i32),
    z:       Vec<i32>,
    charge: i32,
    mag:    i32,
}

fn main() {

    let mut block = LifeBlock {
                                 x_y: (0, 0),
                                 z: Vec::new(),
                                 charge: 0,
                                 mag: 0
                              };

}

fn initialize_life(b: &mut LifeBlock, x: i32, y: i32, z: i32, c: i32, m: i32) -> &LifeBlock {
    b.x_y = (x, y);
    b.z = vec![z];
    b.charge = c;
    b.mag = m;
    b
}

#[test]
fn it_can_begin() {
    let mut block = LifeBlock {
                                 x_y: (0, 0),
                                 z: Vec::new(),
                                 charge: 0,
                                 mag: 0
                              };

    initialize_life(&mut block, 0, 0, 0, 0, 0);

    assert_eq!(block.x_y, (0, 0));
    assert_eq!(block.z, [0]);
    assert_eq!(block.charge, 0);
    assert_eq!(block.mag, 0);
}

#[test]
fn it_can_make_another_life_block_in_a_new_position() {
    let mut block = LifeBlock {
                                 x_y: (0, 0),
                                 z: Vec::new(),
                                 charge: 0,
                                 mag: 0
                              };

    initialize_life(&mut block, 0, 1, 0, 0, 0);

    assert_eq!(block.x_y, (0, 1));
    assert_eq!(block.z, [0]);
    assert_eq!(block.charge, 0);
    assert_eq!(block.mag, 0);
}
