use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub enum CacheType {
    FIFO, 
    FWF, 
    LRU, 
    LFU, 
    RAND, 
    RMA
}


pub struct Cache {
    n            : usize,
    k            : usize,
    cache_type   : CacheType,
    step_counter : usize,
    aux_counter  : usize,
    cache        : Vec<usize>,
    add_mem      : Vec<usize>,
    rnd_gen      : Option<rand::rngs::ThreadRng>
}

impl Cache {
    pub fn new(n: usize, k : usize, cache_type : CacheType) -> Self {
        match cache_type {
            CacheType::FIFO => {
                Cache {
                    n,
                    k,
                    cache_type,
                    step_counter : 0,
                    aux_counter : 0,
                    cache    : vec![0; k],
                    add_mem  : Vec::new(),
                    rnd_gen  : None
                }
            },
            CacheType::FWF | CacheType::LRU => {
                Cache {
                    n,
                    k,
                    cache_type,
                    step_counter : 0,
                    aux_counter : 0,
                    cache    : vec![0; n],
                    add_mem  : Vec::new(),
                    rnd_gen  : None
                }
            },
            _ => {
                Cache {
                    n,
                    k,
                    cache_type,
                    step_counter : 0,
                    aux_counter : 0,
                    cache    : vec![0; k],
                    add_mem  : Vec::new(),
                    rnd_gen  : Some(thread_rng())
                }
            },
        }
    }

    fn fifo_cost(&mut self, page: usize) -> usize {
        if self.cache.contains(&page) {
            0
        } else {
            self.cache[self.aux_counter] = page;
            self.aux_counter = (self.aux_counter + 1) % self.k;
            1
        }
    }

    fn fwf_cost(&mut self, page: usize) -> usize {
        if self.cache[page] == 1 {
            0
        } else {
            if self.aux_counter == self.k {
                self.cache = vec![0; self.n];
                self.cache[page] = 1;
                self.aux_counter = 1;
            } else {
                self.cache[page] = 1;
                self.aux_counter += 1;
            }
            1
        }
    }

    fn lru_cost(&mut self, page: usize) -> usize {       
        if self.cache[page] == 1 {
            0
        } else {
            self.step_counter += 1;
            if self.aux_counter == self.k {
                self.cache = vec![0; self.n];
                self.cache[page] = 1;
                self.aux_counter = 1;
            } else {
                self.cache[page] = 1;
                self.aux_counter += 1;
            }
            1
        }
    }

    pub fn page_cost (&mut self, page : usize) -> usize {
        
        match self.cache_type {
            CacheType::FIFO => self.fifo_cost(page),
            CacheType::FWF  => self.fwf_cost(page),
            CacheType::LRU  => self.lru_cost(page),
            _ => 1,
        }
    }
}
