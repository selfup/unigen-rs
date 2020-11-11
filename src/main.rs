extern crate rand;

use std::env;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;

mod builder;
use builder::Blocks;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut size = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        size = args[1].clone();
    }

    let parsed_size = size.trim().parse::<u32>().unwrap();

    println!("Building Universe..");

    let mut universe = vec![];
    let mut neturon: [u32; 1] = [0];
    let mut proton: [u32; 1] = [0];
    let mut electron: [u32; 1] = [0];

    let mut generated_universe = Blocks::initialize_universe(parsed_size, &mut universe);

    generated_universe = Blocks::tick(parsed_size, &mut generated_universe);
    Blocks::particles(&mut generated_universe, &mut neturon, &mut proton, &mut electron);

    println!("Snapshot..\n\n{:?}\n", &generated_universe[0]);
    println!("Universe built!\nChecking the charge..");

    Blocks::charge_of_field(&mut proton, &mut electron, parsed_size);
    Blocks::atom_charge(&mut generated_universe);

    println!("Size of Universe: {:?}", generated_universe.len());

    for block in generated_universe {
        let y = block.y as f32;
        let x = block.x as f32;
        let z = block.z as f32;

        let mut r = block.charge as f32;

        if r < 0.0 {
            r = 2.0;
        }

        commands
        // cube
            .spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.15, subdivisions: 1 })),
                material: materials.add(Color::rgb(r, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..Default::default()
            });
    }

    commands
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-40.0, 30.0, 30.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}
