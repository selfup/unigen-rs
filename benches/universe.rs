use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::{hint::black_box, mem::size_of, time::Duration};
use unigen::builder::{
    Blocks,
    core::{Block, neutron::Neutrons, proton::Protons},
};

fn populated_universe(size: u32) -> Vec<Block> {
    let mut blocks = Blocks::initialize_universe(size);
    for (i, block) in blocks.iter_mut().enumerate() {
        block.atom.electrons = 59;
        block.atom.nucleus.baryon.protons = Protons::new([0, 59, 117][i % 3]);
        block.atom.nucleus.baryon.neutrons = Neutrons::new((i % 118) as u8);
    }
    blocks
}

fn universe(c: &mut Criterion) {
    // Initialize the global pool and warm each worker's production RNG before timing.
    rayon::broadcast(|_| {
        let mut blocks = Blocks::initialize_universe(1);
        Blocks::tick(&mut blocks);
        black_box(blocks);
    });
    let threads = rayon::current_num_threads();
    eprintln!(
        "Rayon workers: {threads}; bytes per block: {}",
        size_of::<Block>()
    );

    let mut sizes = vec![5_u32, 10, 20];
    if std::env::var("UNIGEN_BENCH_LARGE").as_deref() == Ok("1") {
        sizes.push(50);
    }
    // Include the worker count so baselines from different pools stay separate.
    let mut group = c.benchmark_group(format!("universe/{threads}_threads"));
    for size in sizes {
        let count = u64::from(size).pow(3);
        group.throughput(Throughput::Elements(count));
        eprintln!(
            "side {size}: {count} blocks, {} bytes per universe",
            count * size_of::<Block>() as u64
        );

        group.bench_with_input(BenchmarkId::new("initialize", size), &size, |b, &size| {
            // Include allocation and deallocation; retain only one output at a time.
            b.iter(|| black_box(Blocks::initialize_universe(black_box(size))));
        });

        let fixture = populated_universe(size);
        group.bench_with_input(BenchmarkId::new("tick", size), &fixture, |b, fixture| {
            b.iter_batched_ref(
                || fixture.clone(),
                |blocks| {
                    Blocks::tick(black_box(blocks));
                    black_box(blocks);
                },
                BatchSize::PerIteration,
            );
        });
        group.bench_with_input(
            BenchmarkId::new("particles", size),
            &fixture,
            |b, fixture| {
                let mut blocks = fixture.clone();
                let (mut neutrons, mut protons, mut electrons) = ([0], [0], [0]);
                b.iter(|| {
                    Blocks::particles(
                        black_box(&mut blocks),
                        &mut neutrons,
                        &mut protons,
                        &mut electrons,
                    );
                    black_box((neutrons, protons, electrons))
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("atom_charge", size),
            &fixture,
            |b, fixture| {
                b.iter_batched_ref(
                    || fixture.clone(),
                    |blocks| {
                        Blocks::atom_charge(black_box(blocks));
                        black_box(blocks);
                    },
                    BatchSize::PerIteration,
                );
            },
        );
        drop(fixture);

        group.bench_with_input(BenchmarkId::new("pipeline", size), &size, |b, &size| {
            b.iter(|| {
                let mut blocks = Blocks::initialize_universe(black_box(size));
                let (mut neutrons, mut protons, mut electrons) = ([0], [0], [0]);
                Blocks::tick(&mut blocks);
                Blocks::particles(&mut blocks, &mut neutrons, &mut protons, &mut electrons);
                let charge = Blocks::charge_of_field(&mut protons, &mut electrons, size);
                Blocks::atom_charge(&mut blocks);
                black_box((blocks, neutrons, protons, electrons, charge))
            });
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3))
        .sample_size(30);
    targets = universe
}
criterion_main!(benches);
