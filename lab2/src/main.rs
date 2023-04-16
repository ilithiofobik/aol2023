mod cache;
mod distribution;

use cache::*;
use distribution::*;

fn experiment() {
    let mut rand = rand::thread_rng();
    let num_of_tests = 100_000;
    let ns = [20, 30, 40, 50, 60, 70, 80, 90, 100];
    let cache_types = [
        CacheType::Fifo, 
        CacheType::Fwf, 
        CacheType::Lru,
        CacheType::Lfu,
        CacheType::Rand,
        CacheType::Rma
    ];

    for n in ns {
        let mut distributions = [
                Distribution::Uni(UniDistribution::new(n)),
                Distribution::Har(ArrDistribution::harmonic(n)),
                Distribution::Bih(ArrDistribution::biharmonic(n)),
                Distribution::Geo(GeoDistribution::new(0.5, n))
            ];
        let ks = ((n / 10)..=(n / 5)).collect::<Vec<usize>>();
        for k in ks {   
            for distribution in distributions.iter_mut() {
                for cache_type in cache_types {
                    let mut cache = Cache::new(n, k, cache_type); 
                    let mut sum = 0;
                    for _ in 0..num_of_tests {
                        sum += cache.get_page(distribution.generate(), &mut rand);
                    }
                    // println!(
                    //     "n: {}, k: {}, distribution: {}, cache: {}, cost: {}", 
                    //     n, 
                    //     k, 
                    //     distribution.name(), 
                    //     cache_type.name(), 
                    //     sum as f64 / num_of_tests as f64);
                }
            }
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    experiment();
    println!("Time elapsed: {} s", now.elapsed().as_secs_f64());
}
