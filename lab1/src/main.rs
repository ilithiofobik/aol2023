mod distribution;

use distribution::Distribution;

fn main() {
    let mut x = Distribution::harmonic(100);
    for _ in 0..100 {
        println!("{}", x.generate());
    }
}
