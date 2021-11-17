extern crate rand;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::tasks::prelude::*;
use bevy::tasks::ParallelIterator;
use std::env;

const CHUNK_SIZE: usize = 128;
const DEFAULT_SIZE: u32 = 30;

use unigen::builder;

type Material = StandardMaterial;

#[allow(unused_imports)]

struct ChargeMaterials {
    negative_mat: Handle<Material>,
    positive_mats: Vec<Handle<Material>>,
}

impl ChargeMaterials {
    fn new(mut asset_server: ResMut<Assets<Material>>) -> Self {
        Self {
            negative_mat: asset_server.add(Color::rgb(2.0, 0., 1.).into()),
            positive_mats: {
                (0..std::i8::MAX)
                    .map(|r| asset_server.add(Color::rgb(r as f32, 0., 1.).into()))
                    .collect()
            },
        }
    }

    fn get(&self, r: i8) -> &Handle<Material> {
        if r < 0 {
            &self.negative_mat
        } else {
            &self.positive_mats[r as usize]
        }
    }
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(update_block_atoms.system())
        .add_system(update_block_spheres.system())
        .add_system(camera_movement.system())
        .add_system(random_movement.system())
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

struct CameraMatcher();

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<Material>>,
) {
    let parsed_size: u32 = if let Some(arg) = env::args().nth(1) {
        arg.trim().parse().unwrap()
    } else {
        DEFAULT_SIZE
    };

    let charged_mats = ChargeMaterials::new(materials);

    let blocks = builder::generate_universe(parsed_size);

    let mesh_handle = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.15,
        subdivisions: 1,
    }));

    for block in blocks {
        let y = block.y as f32;
        let x = block.x as f32;
        let z = block.z as f32;

        let r = block.charge;

        commands
            .spawn(PbrBundle {
                mesh: mesh_handle.clone(),
                material: charged_mats.get(r).clone(),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..Default::default()
            })
            .with(block);
    }

    commands.insert_resource(charged_mats);

    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-60.0, 50.0, 50.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraMatcher());
}

fn update_block_spheres(
    pool: Res<ComputeTaskPool>,
    mats: Res<ChargeMaterials>,
    mut query: Query<(&mut Handle<StandardMaterial>, &builder::core::Block)>,
) {
    query
        .par_iter_mut(CHUNK_SIZE)
        .for_each(&pool, |(mut material_handle, block)| {
            let r = block.charge;
            *material_handle = mats.get(r).clone();
        });
}

fn update_block_atoms(pool: Res<ComputeTaskPool>, mut query: Query<&mut builder::core::Block>) {
    query.par_iter_mut(CHUNK_SIZE).for_each(&pool, |mut block| {
        builder::mutate_blocks_with_new_particles(&mut rand::thread_rng(), &mut block);

        builder::calculate_charge(&mut block);
    });
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraMatcher)>,
) {
    let input_dir = get_input_dir(keyboard_input);

    if input_dir.length() > 0. {
        for (mut transform, _camera) in query.iter_mut() {
            let input_dir = (transform.rotation * input_dir).normalize();

            transform.translation += input_dir * (time.delta_seconds_f64() * 50.0) as f32;
        }
    }
}

fn random_movement(
    pool: Res<ComputeTaskPool>,
    mut query: Query<(&mut Transform, &builder::core::Block)>,
) {
    query
        .par_iter_mut(CHUNK_SIZE)
        .for_each(&pool, |(mut transform, block)| {
            let new_translation = Vec3::new(block.x as f32, block.y as f32, block.z as f32);

            transform.translation = new_translation;
        });
}

fn get_input_dir(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut input_dir = Vec3::default();

    if keyboard_input.pressed(KeyCode::W) {
        let forward = Vec3::unit_z();
        input_dir -= forward;
    }

    if keyboard_input.pressed(KeyCode::S) {
        let forward = Vec3::unit_z();
        input_dir += forward;
    }

    if keyboard_input.pressed(KeyCode::A) {
        let right = Vec3::unit_x();
        input_dir -= right;
    }

    if keyboard_input.pressed(KeyCode::D) {
        let right = Vec3::unit_x();
        input_dir += right;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        let up = Vec3::unit_y();
        input_dir += up;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        let up = Vec3::unit_y();
        input_dir -= up;
    }

    input_dir
}
