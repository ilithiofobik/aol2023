mod page_migration;
mod distribution;

use page_migration::*;
use distribution::*;

use std::fs::File;
use std::io::prelude::*;

const N: usize = 64;
const D: u64 = 32;
const REQ_LEN : usize = 1_024;
const NUM_OF_TESTS : usize = 100_000;

fn experiment() {
    let page_structs =
        [
            PageStructure::Hypercube(HypercubeStruct::new()),
            PageStructure::Torus(TorusStruct::new())
        ];

    let mut migrations = 
        [
            Migration::RandomFlip(RandomFlipMigration::new(D)),
            Migration::MoveToMin(MoveToMinMigration::new(D))
        ];
    
    let mut distributions = [
            Distribution::Uni(UniDistribution::new(N)),
            Distribution::Har(ArrDistribution::harmonic(N)),
            Distribution::Bih(ArrDistribution::biharmonic(N))
        ];

    for distribution in distributions.iter_mut() { 
        let mut sequences = Vec::with_capacity(NUM_OF_TESTS);
        for _ in 0..NUM_OF_TESTS {
            sequences.push(distribution.gen_seq(REQ_LEN));
        }

        for migration in migrations.iter_mut() {
            for page_struct in page_structs.iter() {
                let filename = format!("data/{}_{}_{}.txt", distribution.name(), migration.name(), page_struct.name());
                let mut file = File::create(filename).unwrap();
                let mut page_migration = PageMigration::new(page_struct, migration);

                let mut avg = 0.0;
                for seq in sequences.iter() {
                    for page in seq.iter() {
                        avg += page_migration.migrate(*page) as f64;
                    }
                }
                avg /= NUM_OF_TESTS as f64;

                let line = format!("{}", avg);
                file.write_all(line.as_bytes()).unwrap();
            }
        }
    }    
}

fn main() {
    let now = std::time::Instant::now();
    experiment();
    println!("Time elapsed: {} s", now.elapsed().as_secs_f64());
}
