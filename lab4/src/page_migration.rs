use rand::Rng;
use std::vec;

pub enum DistanceType {
    Hypercube,
    Torus
}

impl DistanceType {
    pub fn name(&self) -> &str {
        match self {
            DistanceType::Hypercube => "hypercube",
            DistanceType::Torus => "torus",
        }
    }
}

pub enum MigrationType {
    RandomFlip,
    MoveToMin
}

impl MigrationType {
    pub fn name(&self) -> &str {
        match self {
            MigrationType::RandomFlip => "random_flip",
            MigrationType::MoveToMin => "move_to_min",
        }
    }
}

pub struct PageMigration {
    distance_type : DistanceType,
    migration_type : MigrationType,
    pub n : usize,
    pub d : u64,
    pub generator : rand::rngs::ThreadRng,
    current_page : u8
}

impl PageMigration {
    pub fn new(distance_type : DistanceType, migration_type : MigrationType, n : usize, d : u64) -> Self {
        PageMigration {
            distance_type,
            migration_type,
            n,
            d,
            generator : rand::thread_rng(),
            current_page : 1
        }
    }

    fn distance(&self, page1 : u8, page2 : u8) -> u64 {
        match self.distance_type {
            DistanceType::Hypercube => 
                ((page1 - 1) ^ (page2 - 1)).count_ones() as u64,
            DistanceType::Torus => {
                let mut x = page1 - 1;
                let mut y = page2 - 1;
                let mut dist = 0;

                for _ in 0..3 {
                    let a = x & 0b11;
                    let b = y & 0b11;

                    match a.abs_diff(b) {
                        1 | 3 => dist += 1,
                        2 => dist += 2,
                        _ => ()
                    }

                    x = x.rotate_right(2);
                    y = y.rotate_right(2);
                }
  
                dist
            }
        }
    }

    fn cost(&self, y: u8) -> u64 {
        self.d * self.distance(self.current_page, y)
    }
}