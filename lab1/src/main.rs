mod distribution;
mod access;

use access::TransList;
use distribution::Distribution;
use access::*;

fn experiment(num_of_tests: usize, dist: &mut Distribution, access: &mut AccessList) -> f64 {
    let mut count = 0;
    for _ in 0..num_of_tests {
        count += access.access(dist.generate());
    }
    count as f64 / num_of_tests as f64
}

fn main() {
    let max_int = 100;
    let test_nums = [100, 500, 1000, 5000, 10000, 50000, 100000];
    
    let distributions = [
        ("uniform",    Distribution::uniform(max_int)),
        ("harmonic",   Distribution::harmonic(max_int)),
        ("biharmonic", Distribution::biharmonic(max_int)),
        ("geometric",  Distribution::geometric(max_int))
    ];

    let access_types = [
        ("SimpleList",  AccessList::SimpleList(SimpleList::new())),
        ("TransList",   AccessList::TransList(TransList::new())),
        ("MoveToFront", AccessList::MtfList(MtfList::new())),
        ("Count",       AccessList::CountList(CountList::new()))

    ];

    for (access_name, access) in access_types.iter() {
        for (dist_name, dist) in distributions.iter() {
            for num in test_nums.iter() {
                let mut dist = (*dist).clone();
                let mut access = (*access).clone();
                println!("Dist: {}, Access: {}, Size: {}, Average: {}", access_name, dist_name, num, experiment(*num, &mut dist, &mut access));
            }
        }
    }
}
