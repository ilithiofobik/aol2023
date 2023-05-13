mod bin_pack;
mod distribution;

use bin_pack::*;
use distribution::*;

use std::fs::File;
use std::io::prelude::*;
use fastrand::Rng;

const NUM_OF_TESTS: usize = 100_000;
const SEQ_LEN: usize = 100;
const N: usize = 10;

fn experiment() {
    let mut bin_pack_types = [
        BinPacking::Next,
        BinPacking::First,
        BinPacking::Best,
        BinPacking::Worst,
        BinPacking::Random(Rng::new())
    ];
    
    let mut distributions = [
        Distribution::Uni(UniDistribution::new(N)),
        Distribution::Har(ArrDistribution::harmonic(N)),
        Distribution::Bih(ArrDistribution::biharmonic(N)),
        Distribution::Geo(GeoDistribution::new(0.5, N))
    ];

    for distribution in distributions.iter_mut() { 
        let mut sequences = Vec::with_capacity(NUM_OF_TESTS);
        for _ in 0..NUM_OF_TESTS {
            sequences.push(distribution.gen_seq(SEQ_LEN));
        }
        for bin_pack_type in bin_pack_types.iter_mut() {
            let filename = format!("data/dist_{}bp_{}.txt", distribution.name(), bin_pack_type.name());
            let mut file = File::create(filename).unwrap();

            let mut avg = 0.0;
            for seq in sequences.iter() {
                let mut bin_pack = BinPack::new(bin_pack_type);
                bin_pack.pack(seq);
                avg += (bin_pack.num_of_bins() as f64) / optimal_packing(seq);
            }
            avg /= NUM_OF_TESTS as f64;

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
