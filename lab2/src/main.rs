mod cache;
mod distribution;

use cache::*;
use distribution::*;

fn experiment() {
    let num_of_tests = 100;
    let ns = [20, 30, 40, 50, 60, 70, 80, 90, 100];
    let cache_types = [
        CacheType::FIFO, 
        CacheType::FWF, 
        CacheType::LRU
    ];

    for n in ns {
        // let distributions = [
        //     Distribution::Uni(UniDistribution::new(n)),
        //     Distribution::Har(ArrDistribution::harmonic(n)),
        //     Distribution::Bih(ArrDistribution::biharmonic(n)),
        //     Distribution::Geo(GeoDistribution::new(0.5, n))
        // ];
        let ks = ((n / 10)..=(n / 5)).collect::<Vec<usize>>();
        for k in ks {
            // for distribution in distributions.into_iter() {
            //     for cache_type in cache_types {
            //         let cache = Cache::new(n, k, cache_type); 
            //         let mut sum = 0;
            //         for _ in 0..num_of_tests {
            //             sum += cache.page_cost((distribution).generate());
            //         }
            //         println!("n: {}, k: {}, distribution: {}, cost: {}", n, k, distribution.name(), sum as f64 / num_of_tests as f64);
            //     }
            // }
        }
    }
}

fn main() {
    experiment();
}
