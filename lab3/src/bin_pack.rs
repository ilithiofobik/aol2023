use fastrand::Rng;
use std::vec;

macro_rules! filtered_pairs {
    ($v:expr, $r: expr) => {
        $v
        .iter()
        .enumerate()
        .filter(|(_, &x)| x + $r <= 1.0)
    };
}

pub enum BinPacking {
    Next,
    First,
    Best,
    Worst,
    Random(Rng),
}

impl BinPacking {
    pub fn name(&self) -> &str {
        match self {
            BinPacking::Next => "NEXT",
            BinPacking::First => "FIRST",
            BinPacking::Best => "BEST",
            BinPacking::Worst => "WORST",
            BinPacking::Random(_) => "RANDOM",
        }
    }

    pub fn find_idx(&mut self, r: f64, bins: &[f64]) -> Option<usize> {
        match self {
            BinPacking::Next => {
                let last = bins.len() - 1;
                if bins[last] + r <= 1.0 {
                    Some(last)
                } else {
                    None
                }
            },
            BinPacking::First => {
                bins.iter().position(|&x| x + r <= 1.0)
            },
            BinPacking::Best => {
                filtered_pairs!(bins, r)
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(i, _)| i)
            },
            BinPacking::Worst => {
                filtered_pairs!(bins, r)
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(i, _)| i)
            },
            BinPacking::Random(rand) => {
                let len = 
                    filtered_pairs!(bins, r)
                    .clone()
                    .count();
        
                if len > 0 {
                    filtered_pairs!(bins, r)
                    .nth(rand.usize(0..len))
                    .map(|(i, _)| i)
                } else {
                    None
                }
            }
        }
    }
}

pub struct BinPack<'a> {
    bins : Vec<f64>,
    bin_pack : &'a mut BinPacking
}

impl<'a> BinPack<'a> {
    pub fn new(bin_pack : &'a mut BinPacking) -> Self {
        let bins = vec![ 0.0 ];
        BinPack {
            bins,
            bin_pack
        }
    }

    pub fn add_val(&mut self, r: f64) {
        let idx_opt = self.bin_pack.find_idx(r, &self.bins);

        match idx_opt {
            Some(idx) => self.bins[idx] += r,
            None => self.bins.push(r),
        }
    }

    pub fn pack(&mut self, rs: &[f64]) {
        for r in rs {
            self.add_val(*r);
        }
    }

    pub fn num_of_bins(&self) -> usize {
        self.bins.len()
    }
}

pub fn optimal_packing(arr : &[f64]) -> f64 {
    arr.iter().sum::<f64>().ceil()
}