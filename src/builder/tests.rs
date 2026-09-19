use super::core::{neutron::NeutronData, proton::ProtonData};
use super::*;
use rand::{SeedableRng, rngs::StdRng};

fn populated_blocks(len: usize) -> Vec<Block> {
    let template = Blocks::initialize_universe(1)[0];
    (0..len)
        .map(|i| {
            let mut block = template;
            block.id = i as u32;
            block.x = i as i32;
            block.y = -(i as i32);
            block.atom.electrons = 59;
            block.atom.nucleus.baryon.protons = Protons::new([0, 59, 118][i % 3]);
            block.atom.nucleus.baryon.neutrons = Neutrons::new(118);
            block.charge = 42; // Sentinel: every block must be processed.
            block
        })
        .collect()
}

#[test]
fn field_charge_depends_only_on_particle_totals() {
    for size in [0, 1, 5, u32::MAX] {
        for (protons, electrons, expected) in [
            (0, 0, Charge::Neutral),
            (1, 1, Charge::Neutral),
            (125, 125, Charge::Neutral),
            (250, 250, Charge::Neutral),
            (u32::MAX, u32::MAX, Charge::Neutral),
            (1, 0, Charge::Cationic),
            (124, 123, Charge::Cationic),
            (125, 124, Charge::Cationic),
            (250, 249, Charge::Cationic),
            (0, 1, Charge::Anionic),
            (124, 125, Charge::Anionic),
            (249, 250, Charge::Anionic),
        ] {
            assert_eq!(
                Blocks::charge_of_field(&mut [protons], &mut [electrons], size),
                expected,
                "protons={protons}, electrons={electrons}, size={size}"
            );
        }
    }
}

#[test]
fn particle_totals_are_exact_and_replace_previous_results() {
    let mut blocks = populated_blocks(129);
    let (mut neutrons, mut protons, mut electrons) = ([999], [999], [999]);
    Blocks::particles(&mut blocks, &mut neutrons, &mut protons, &mut electrons);
    assert_eq!(neutrons, [129 * 118]);
    assert_eq!(protons, [43 * (59 + 118)]);
    assert_eq!(electrons, [129 * 59]);

    Blocks::particles(&mut [], &mut neutrons, &mut protons, &mut electrons);
    assert_eq!((neutrons, protons, electrons), ([0], [0], [0]));
}

#[test]
fn atom_charge_handles_all_signs_and_chunk_boundaries() {
    for len in [0, 1, 127, 128, 129, 257] {
        let mut blocks = populated_blocks(len);
        Blocks::atom_charge(&mut blocks);
        for (i, block) in blocks.iter().enumerate() {
            assert_eq!(block.charge, [-1, 0, 1][i % 3], "len={len}, block={i}");
        }
    }
}

fn assert_tick_invariants(before: &Block, after: &Block) {
    assert_eq!(after.id, before.id);
    let distance =
        (after.x - before.x).abs() + (after.y - before.y).abs() + (after.z - before.z).abs();
    assert_eq!(distance, 1, "block {} must move exactly one step", after.id);
    assert!(after.atom.electrons < 118);

    let baryon = &after.atom.nucleus.baryon;
    assert!(baryon.protons.count < 118);
    assert!(baryon.neutrons.count < 118);
    for (i, particle) in baryon.protons.protons.iter().enumerate() {
        let expected = if i < baryon.protons.count as usize {
            ProtonData::RedGreenBlueUpUpDownQuark
        } else {
            ProtonData::Unknown
        };
        assert_eq!(*particle, expected);
    }
    for (i, particle) in baryon.neutrons.neutrons.iter().enumerate() {
        let expected = if i < baryon.neutrons.count as usize {
            NeutronData::RedGreenBlueUpDownDownQuark
        } else {
            NeutronData::Unknown
        };
        assert_eq!(*particle, expected);
    }
}

#[test]
fn seeded_mutation_preserves_particle_and_movement_invariants() {
    let mut rng = StdRng::seed_from_u64(42);
    let mut block = populated_blocks(1)[0];
    for _ in 0..1024 {
        let before = block;
        mutate_blocks_with_new_particles(&mut rng, &mut block);
        assert_tick_invariants(&before, &block);
    }
}

#[test]
fn tick_processes_every_block_across_chunk_boundaries() {
    for len in [0, 1, 127, 128, 129, 257] {
        let mut blocks = populated_blocks(len);
        for _ in 0..2 {
            let before = blocks.clone();
            Blocks::tick(&mut blocks);
            for (old, new) in before.iter().zip(&blocks) {
                assert_tick_invariants(old, new);
            }
        }
    }
}

#[test]
fn initialization_assigns_unique_ids_and_coordinates_in_order() {
    assert!(Blocks::initialize_universe(0).is_empty());
    let blocks = Blocks::initialize_universe(5);
    let mut expected_id = 0;
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                let block = &blocks[expected_id];
                assert_eq!(block.id, expected_id as u32);
                assert_eq!((block.x, block.y, block.z), (x, y, z));
                assert_eq!(block.charge, 0);
                assert_eq!(block.atom.electrons, 0);
                assert_eq!(block.atom.nucleus.baryon.protons.count, 0);
                assert_eq!(block.atom.nucleus.baryon.neutrons.count, 0);
                expected_id += 1;
            }
        }
    }
    assert_eq!(blocks.len(), expected_id);
}
