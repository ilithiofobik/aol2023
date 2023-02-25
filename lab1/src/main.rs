mod distribution;
mod access;

use distribution::Distribution;
use access::CountList;
use access::Accessible;

fn main() {
    let mut x = Distribution::geometric(100);
    let mut y = CountList::new();
    for _ in 0..10000 {
        println!("{}", y.access(x.generate()));
    }
}
