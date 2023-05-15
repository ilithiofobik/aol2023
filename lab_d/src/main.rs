mod page_allocation;

use page_allocation::*;

use std::fs::File;
use std::io::Write;

const NUM_OF_TESTS_U : usize = 1000;
const NUM_OF_TESTS_F : f64 = NUM_OF_TESTS_U as f64;

fn experiment() {
    let ds = [16, 32, 64, 128, 256];
    let ps = [0.01, 0.02, 0.05, 0.1, 0.2, 0.5];

    for d in ds {
        for p in ps {
            let filename = format!("data/d_{}_p_{}_allocation.txt", d, p);
            let mut file = File::create(filename).unwrap();

            let mut avg_cost = 0.0;
            let mut avg_max_copies = 0.0;

            for _ in 0..NUM_OF_TESTS_U {
                let (cost, copies) = page_allocation(d, p);
                avg_cost += cost;
                avg_max_copies += copies;
            }

            avg_cost /= NUM_OF_TESTS_F;
            avg_max_copies /= NUM_OF_TESTS_F;

            let line = format!("{}, {}", avg_cost, avg_max_copies);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let timer = std::time::Instant::now();
    experiment();
    println!("Time elapsed: {:?}", timer.elapsed());
}