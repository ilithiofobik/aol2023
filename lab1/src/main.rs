mod distribution;
mod access;
mod plots;

use std::collections::HashMap;

use access::TransList;
use distribution::*;
use access::*;
use plots::*;

fn experiment(num_of_tests: usize, dist: &mut Distribution, access: &mut AccessList) -> f64 {
    let mut count = 0;
    for _ in 0..num_of_tests {
        count += access.access(dist.generate());
    }
    count as f64 / num_of_tests as f64
}

fn multi_experiment(num_of_exps: usize, num_of_tests: usize, dist: &mut Distribution, access: &mut AccessList) -> f64 {
    let mut count = 0.0;
    for _ in 0..num_of_exps {
        let mut access = (*access).clone();
        count += experiment(num_of_tests, dist, &mut access);
    }
    count / num_of_exps as f64
}

fn main() {
    let max_int = 100;
    let test_nums = [100, 500, 1_000, 5_000, 10_000, 50_000, 100_000];
    let num_of_exps = 1;
    
    let distributions = [
        Distribution::Uni(UniDistribution::new(max_int)),
        Distribution::Har(ArrDistribution::harmonic(max_int)),
        Distribution::Bih(ArrDistribution::biharmonic(max_int)),
        Distribution::Geo(GeoDistribution::new(0.5, max_int))
    ];

    let access_types = [
        AccessList::Simple(SimpleList::new()),
        AccessList::Trans(TransList::new()),
        AccessList::Mtf(MtfList::new()),
        AccessList::Count(CountList::new())
    ];

    let mut plot_data = HashMap::<(String, String), Vec<f64>>::new();

    for access in access_types.iter() {
        for dist in distributions.iter() {
            let y_data : Vec<f64> = 
                test_nums
                .into_iter()
                .map(|num| -> f64 {
                    let mut access = (*access).clone();
                    let mut dist = (*dist).clone();
                    multi_experiment(num_of_exps, num, &mut dist, &mut access)
                }
                )
                .collect();

            let access_name = access.name();
            let dist_name = dist.name();
            
            plot_data.insert((access_name, dist_name), y_data);
        }
    }

    plot().unwrap();
}
