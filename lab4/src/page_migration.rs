use fastrand::Rng;

#[derive(Clone)]
pub enum PageStructure {
    Hypercube,
    Torus
}

impl PageStructure {
    pub fn name (&self) -> &str {
        match self {
            PageStructure::Hypercube => "hypercube",
            PageStructure::Torus     => "torus"
        }
    }

    pub fn distance(&self, page1 : u8, page2 : u8) -> u64 {
        match self {
            PageStructure::Hypercube => 
                ((page1 - 1) ^ (page2 - 1)).count_ones() as u64,
                
            PageStructure::Torus => {
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
}

#[derive(Clone)]
pub struct RandomFlipMigration {
    d : u64,
    generator : Rng,
    current_page : u8
}

impl RandomFlipMigration {
    pub fn new(d : u64) -> Self {
        RandomFlipMigration {
            d,
            generator : Rng::new(),
            current_page : 1
        }
    }

    pub fn migrate(&mut self, page : u8, page_struct : &PageStructure) -> u64 {
        let cost = self.d * page_struct.distance(self.current_page, page);
        if self.generator.u64(0..2 * self.d) == 0 {
            self.current_page = page;
        }
        cost
    }
}

#[derive(Clone)]
pub struct MoveToMinMigration {
    d : u64,
    current_page : u8, 
    requests : Vec<u8>
}

impl MoveToMinMigration {
    pub fn new(d : u64) -> Self {
        MoveToMinMigration {
            d,
            current_page : 1, 
            requests : Vec::with_capacity(d as usize)
        }
    }

    pub fn migrate(&mut self, page : u8, page_struct : &PageStructure) -> u64 {
        let cost = self.d * page_struct.distance(self.current_page, page);
        self.requests.push(page);

        if self.requests.len() == self.d as usize {
            let mut min_cost = u64::MAX;
            let mut min_page = 0;

            for page in 1..=64 {
                let cost = 
                    self.requests.iter()
                    .fold(0, |acc, x| acc + page_struct.distance(*x, page));

                if cost < min_cost {
                    min_cost = cost;
                    min_page = page;
                }
            }

            self.current_page = min_page;
            self.requests.clear();
        }

        cost
    }
}

#[derive(Clone)]
pub enum Migration {
    RandomFlip(RandomFlipMigration),
    MoveToMin(MoveToMinMigration)
}

impl Migration {
    pub fn name(&self) -> &str {
        match self {
            Migration::RandomFlip(_) => "random_flip",
            Migration::MoveToMin(_) => "move_to_min",
        }
    }

    pub fn migrate(&mut self, page : u8, page_struct : &PageStructure) -> u64 {
        match self {
            Migration::RandomFlip(x) => x.migrate(page, page_struct),
            Migration::MoveToMin(x) => x.migrate(page, page_struct)
        }
    }
}

pub struct PageMigration<'a> {
    page_struct : &'a PageStructure,
    migration : &'a mut Migration,
}

impl<'a> PageMigration<'a> {
    pub fn new(page_struct : &'a PageStructure, migration : &'a mut Migration) -> Self {
        PageMigration {
            page_struct,
            migration
        }
    }

    pub fn migrate(&mut self, page : u8) -> u64 {
        self.migration.migrate(page, self.page_struct)
    }
}