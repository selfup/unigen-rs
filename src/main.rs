struct LifeBlock {
    x_y: Vec<[i32;2]>,
    z:       Vec<i32>,
}

fn main() {
    let mut block = LifeBlock {x_y: Vec::new(), z: Vec::new()};

    initialize_life(&mut block);
}

fn initialize_life(b: &mut LifeBlock) -> &LifeBlock {
    let origin : [i32;2] = [0, 0];
    b.x_y = vec![origin];
    b.z = vec![0];
    b
}

#[test]
fn it_can_begin() {
    let mut block = LifeBlock {x_y: Vec::new(), z: Vec::new()};

    initialize_life(&mut block);

    assert_eq!(block.x_y, [[0, 0]]);
    assert_eq!(block.z, [0]);
}
