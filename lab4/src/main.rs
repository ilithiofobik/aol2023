mod page_migration;
mod distribution;

use page_migration::*;
use distribution::*;

use std::fs::File;
use std::io::prelude::*;

const N: usize = 64;
const D: usize = 32;

fn experiment() {
    let num_of_tests = 100_000;
    let seq_len = 100;

    let bin_pack_types = [
        BinPackType::Next,
        BinPackType::First,
        BinPackType::Best,
        BinPackType::Worst,
        BinPackType::Random
    ];
    
    let mut distributions = [
            Distribution::Uni(UniDistribution::new(N)),
            Distribution::Har(ArrDistribution::harmonic(N)),
            Distribution::Bih(ArrDistribution::biharmonic(N))
        ];

    for distribution in distributions.iter_mut() { 
        let mut sequences = Vec::with_capacity(num_of_tests);
        for _ in 0..num_of_tests {
            sequences.push(distribution.gen_seq(seq_len));
        }
        for bin_pack_type in bin_pack_types {
            let filename = format!("data/dist_{}bp_{}.txt", distribution.name(), bin_pack_type.name());
            let mut file = File::create(filename).unwrap();

            let mut avg = 0.0;
            for seq in sequences.iter() {
                let mut bin_pack = BinPack::new(bin_pack_type);
                bin_pack.pack(seq);
                avg += (bin_pack.bins.len() as f64) / optimal_packing(seq);
            }
            avg /= num_of_tests as f64;

            let line = format!("{}", avg);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    
}

fn main() {
    let now = std::time::Instant::now();
    experiment();
    println!("Time elapsed: {} s", now.elapsed().as_secs_f64());
}
