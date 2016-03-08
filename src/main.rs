struct LifeBlock {
    x_and_y: Vec<[i32;2]>,
    z:       Vec<i32>,
}

fn main() {
    let mut structure = LifeBlock {x_and_y: Vec::new(), z: Vec::new()};

    let std : [i32;2] = [0, 0];

    structure.x_and_y = vec![std];
    structure.z       = vec![0];

    identify_structure(&mut structure.x_and_y, &mut structure.z);
}

fn identify_structure(x_and_y: &mut Vec<[i32;2]>, z: &mut Vec<i32>) {
    println!("{:?}", x_and_y);
    println!("{:?}", z);
}
