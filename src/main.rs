extern crate rand;

use std::env;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;

mod builder;
use builder::Blocks;

#[allow(unused_imports)]
use rand::Rng;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(update_odd_block_atoms.system())
        .add_system(update_even_block_atoms.system())
        .add_system(update_odd_block_spheres.system())
        .add_system(update_even_block_spheres.system())
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let blocks = generate_universe();

    for block in blocks {
        let y = block.y as f32;
        let x = block.x as f32;
        let z = block.z as f32;

        let mut r = block.charge as f32;

        if r < 0.0 {
            r = 2.0;
        }

        commands
            .spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.15, subdivisions: 1 })),
                material: materials.add(Color::rgb(r, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..Default::default()
            })
            .with(block);
    }

    commands
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-60.0, 50.0, 50.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

fn update_even_block_spheres(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Handle<StandardMaterial>, &builder::core::Block)>,
) {
    for (material_handle, block) in query.iter_mut() {
        if block.id % 2 == 0 {
            update_albedo(&mut materials, material_handle, block);
        }
    }
}

fn update_odd_block_spheres(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Handle<StandardMaterial>, &builder::core::Block)>,
) {
    for (material_handle, block) in query.iter_mut() {
        if block.id % 2 != 0 {
            update_albedo(&mut materials, material_handle, block);
        }
    }
}

fn update_albedo(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    material_handle: &Handle<StandardMaterial>,
    block: &builder::core::Block,
) {
    let mut material = materials.get_mut(material_handle).unwrap();
            
    let mut r = block.charge as f32;

    if r < 0.0 {
        r = 2.0;
    }

    material.albedo = Color::rgb(r, 0.7, 0.6).into();
}

fn update_odd_block_atoms(
    mut query: Query<&mut builder::core::Block>,
) {
    for mut block in query.iter_mut() {
        if block.id % 2 != 0 {
            let mut rng = rand::thread_rng();
    
            builder::mutate_blocks_with_new_particles(&mut rng, &mut block);
    
            builder::calculate_charge(&mut block);
        }
    }
}

fn update_even_block_atoms(
    mut query: Query<&mut builder::core::Block>,
) {
    for mut block in query.iter_mut() {
        if block.id % 2 == 0 {
            let mut rng = rand::thread_rng();
    
            builder::mutate_blocks_with_new_particles(&mut rng, &mut block);
    
            builder::calculate_charge(&mut block);
        }
    }
}

fn generate_universe() -> Vec<builder::core::Block> {
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

    generated_universe
}
