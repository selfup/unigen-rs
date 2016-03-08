use std::cell::Cell;

struct LifeBlock {
    x_y: Vec<[i32;2]>,
    z:       Vec<i32>,
    charge: Cell<i32>,
    mag:    Cell<i32>,
}

fn main() {
    let mut block = LifeBlock {
                                 x_y: Vec::new(),
                                 z: Vec::new(),
                                 charge: Cell::new(0),
                                 mag: Cell::new(0)
                              };

    initialize_life(&mut block, 0, 0, 0, 0, 0);
}

fn initialize_life(b: &mut LifeBlock, x: i32, y: i32, z: i32, c: i32, m: i32) -> &LifeBlock {
    let origin : [i32;2] = [x, y];
    b.x_y = vec![origin];
    b.z = vec![z];
    b.charge.set(c);
    b.mag.set(m);
    b
}

#[test]
fn it_can_begin() {
    let mut block = LifeBlock {
                                 x_y: Vec::new(),
                                 z: Vec::new(),
                                 charge: Cell::new(0),
                                 mag: Cell::new(0)
                              };

    initialize_life(&mut block, 0, 0, 0, 0, 0);

    assert_eq!(block.x_y, [[0, 0]]);
    assert_eq!(block.z, [0]);
    assert_eq!(block.charge.get(), 0);
    assert_eq!(block.mag.get(), 0);
}

#[test]
fn it_can_make_another_life_block_in_a_new_position() {
    let mut block = LifeBlock {
                                 x_y: Vec::new(),
                                 z: Vec::new(),
                                 charge: Cell::new(0),
                                 mag: Cell::new(0)
                              };

    initialize_life(&mut block, 0, 1, 0, 0, 0);

    assert_eq!(block.x_y, [[0, 1]]);
    assert_eq!(block.z, [0]);
    assert_eq!(block.charge.get(), 0);
    assert_eq!(block.mag.get(), 0);
}
