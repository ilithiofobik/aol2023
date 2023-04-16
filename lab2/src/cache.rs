use rand::{Rng};
use std::{collections::HashSet, vec};

#[derive(Clone, Copy)]
pub enum CacheType {
    Fifo, 
    Fwf, 
    Lru, 
    Lfu, 
    Rand, 
    Rma
}

impl CacheType {
    pub fn name(&self) -> &str {
        match self {
            CacheType::Fifo => "FIFO",
            CacheType::Fwf  => "FWF",
            CacheType::Lru  => "LRU",
            CacheType::Lfu  => "LFU",
            CacheType::Rand => "RAND",
            CacheType::Rma  => "RMA"
        }
    }
}

pub struct Cache {
    k            : usize,
    cache_type   : CacheType,
    set_cache    : HashSet<usize>,
    fifo_pointer : usize,
    lru_counter  : usize,
    add_arr      : Vec<usize>,
}

impl Cache {
    pub fn new(n: usize, k : usize, cache_type : CacheType) -> Self {
        let add_arr = match cache_type {
            CacheType::Fifo => vec![0; k], // to keep track of the order of pages
            CacheType::Lru | CacheType::Lfu | CacheType::Rma => vec![0; n + 1], // to keep additional info
            _ => vec![0; 0]
        };

        Cache {
            k,
            cache_type,
            set_cache : HashSet::new(),
            fifo_pointer : 0,
            add_arr,
            lru_counter : 0,
        }
    }

    fn fifo_add(&mut self, page: usize) {
        // removing
        let to_remove = self.add_arr[self.fifo_pointer];
        self.set_cache.remove(&to_remove);
        //adding
        self.add_arr[self.fifo_pointer] = page;
        self.set_cache.insert(page);
        
        self.fifo_pointer = (self.fifo_pointer + 1) % self.k;
    }

    fn fwf_add(&mut self, page: usize) {
        // removing
        if self.set_cache.len() == self.k {
            self.set_cache.clear();
        }
        // adding
        self.set_cache.insert(page);
    }

    fn rand_add(&mut self, page: usize, rand: &mut rand::rngs::ThreadRng) {
        // removing
        if self.set_cache.len() == self.k {
            let rand_idx = (*rand).gen_range(0..self.k);
            let rand_page = *self.set_cache.iter().nth(rand_idx).unwrap();
            self.set_cache.remove(&rand_page);
        }
        // adding
        self.set_cache.insert(page);
    }

    fn lru_update(&mut self, page: usize) {
        self.lru_counter += 1;
        self.add_arr[page] = self.lru_counter;
    }

    fn lru_add(&mut self, page: usize) {     
        self.lru_update(page);

        // remove
        if self.set_cache.len() == self.k {
            let lr_page =
                *self.set_cache
                .iter()
                .min_by_key(|&v| self.add_arr[*v])
                .unwrap();
            self.set_cache.remove(&lr_page);
        }
        // add
        self.set_cache.insert(page);
    }

    fn lfu_update(&mut self, page: usize) {
        self.add_arr[page] += 1;
    }

    fn lfu_add(&mut self, page: usize) {   
        self.lfu_update(page);

        // remove
        if self.set_cache.len() == self.k {
            let lf_page =
                *self.set_cache
                .iter()
                .min_by_key(|&v| self.add_arr[*v])
                .unwrap();
            self.set_cache.remove(&lf_page);
        }

        // add
        self.set_cache.insert(page);
    }

    fn rma_update(&mut self, page: usize) {
        self.add_arr[page] = 1;
    }

    fn rma_add(&mut self, page: usize, rand: &mut rand::rngs::ThreadRng) { 
        // remove 
        if self.set_cache.len() == self.k {
             // unmark if all marked
            if self.set_cache.iter().all(|&v| self.add_arr[v] == 1) {
                for page in self.set_cache.iter() {
                    self.add_arr[*page] = 0;
                }
            }

            let unmarked_count = self.set_cache.iter().filter(|&v| self.add_arr[*v] == 0).count();
            let rand_idx = (*rand).gen_range(0..unmarked_count);
            let rand_page = *self.set_cache.iter().filter(|&v| self.add_arr[*v] == 0).nth(rand_idx).unwrap();
            self.set_cache.remove(&rand_page);
        }      
       
        self.set_cache.insert(page);        
    }

    /// Returns 1 if page is not in cache, 0 otherwise.
    pub fn get_page (&mut self, page : usize, rand : &mut rand::rngs::ThreadRng) -> usize {
        if self.set_cache.contains(&page) {
            match self.cache_type {
                CacheType::Lru => self.lru_update(page),
                CacheType::Lfu => self.lfu_update(page),
                CacheType::Rma => self.rma_update(page),
                _ => (),
            }
            0
        } else {
            match self.cache_type {
                CacheType::Fifo  => self.fifo_add(page),
                CacheType::Fwf   => self.fwf_add(page),
                CacheType::Rand  => self.rand_add(page, rand),
                CacheType::Lru   => self.lru_add(page),
                CacheType::Lfu   => self.lfu_add(page),
                CacheType::Rma   => self.rma_add(page, rand),
            }
            1
        }
    }
}
