extern crate rand;

use std::env;
use bevy::prelude::*;

mod builder;

fn main() {
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

    let mut generated_universe = builder::initialize_universe(parsed_size, &mut universe);

    generated_universe = builder::tick(parsed_size, &mut generated_universe);
    builder::particles(&mut generated_universe, &mut neturon, &mut proton, &mut electron);

    println!("Snapshot..\n\n{:?}\n", &generated_universe[0]);
    println!("Universe built!\nChecking the charge..");

    builder::charge_of_field(&mut proton, &mut electron, parsed_size);
    builder::atom_charge(&mut generated_universe);

    println!("Size of Universe: {:?}", generated_universe.len());

    App::build()
        .add_startup_system(setup.system()) // <--
        .add_startup_stage("game_setup") // <--
        .add_startup_system_to_stage("game_setup", spawn_atom.system())
        .add_plugins(DefaultPlugins)
        .run();
}

struct Atom;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}

fn spawn_atom(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteComponents {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Atom);
}
