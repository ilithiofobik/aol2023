use rand::Rng;
use std::vec;

#[derive(Clone, Copy)]
pub enum BinPackType {
    Next,
    First,
    Best,
    Worst,
    Random,
}

impl BinPackType {
    pub fn name(&self) -> &str {
        match self {
            BinPackType::Next => "NEXT",
            BinPackType::First => "FIRST",
            BinPackType::Best => "BEST",
            BinPackType::Worst => "WORST",
            BinPackType::Random => "RANDOM",
        }
    }
}

pub struct BinPack {
    pub bins : Vec<f64>,
    bin_pack : BinPackType,
    rand     : rand::rngs::ThreadRng,
}

impl BinPack {
    pub fn new(bin_pack : BinPackType) -> Self {
        let bins = vec![ 0.0 ];
        let rand = rand::thread_rng();
        BinPack {
            bins,
            bin_pack,
            rand
        }
    }

    pub fn add_val(&mut self, r: f64) {
        let idx_opt = self.find_idx(r);
        self.add_on_idx(r, idx_opt);
    }

    pub fn pack(&mut self, rs: &Vec<f64>) {
        for r in rs {
            self.add_val(*r);
        }
    }

    fn add_on_idx(&mut self, r: f64, idx_opt: Option<usize>) {
        match idx_opt {
            Some(idx) => self.bins[idx] += r,
            None => self.bins.push(r),
        }
    }

    fn find_idx(&mut self, r: f64) -> Option<usize> {
        match self.bin_pack {
            BinPackType::Next => self.next_idx(r),
            BinPackType::First => self.first_idx(r),
            BinPackType::Best => self.best_idx(r),
            BinPackType::Worst => self.worst_idx(r),
            BinPackType::Random => self.random_idx(r),
        }
    }

    fn next_idx(&self, r: f64) -> Option<usize> {
        let last = self.bins.len() - 1;
        if self.bins[last] + r <= 1.0 {
            Some(last)
        } else {
            None
        }
    }

    fn first_idx(&self, r: f64) -> Option<usize> {
        self.bins.iter().position(|&x| x + r <= 1.0)
    }

    fn best_idx(&self, r: f64) -> Option<usize> {       
        self.bins
        .iter()
        .enumerate()
        .filter(|(_, &x)| x + r <= 1.0)
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(i, _)| i)
    }

    fn worst_idx(&self, r: f64) -> Option<usize> {
        self.bins
        .iter()
        .enumerate()
        .filter(|(_, &x)| x + r <= 1.0)
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(i, _)| i)
    }

    fn random_idx(&mut self, r: f64) -> Option<usize> {
        let mut filtered =
            self.bins
            .iter()
            .enumerate()
            .filter(|(_, &x)| x + r <= 1.0);
        
        let len = 
            filtered
            .clone()
            .count();

        if len > 0 {
            filtered
            .nth(self.rand.gen_range(0..len))
            .map(|(i, _)| i)
        } else {
            None
        }
    }
}

pub fn optimal_packing(arr : &[f64]) -> f64 {
    let sum : f64 = arr.iter().sum();
    sum.ceil()
}