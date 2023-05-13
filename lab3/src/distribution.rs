use fastrand::Rng;

pub struct UniDistribution {
    n: usize,
    pub generator: Rng
}

impl UniDistribution {
    pub fn new(n: usize) -> Self {
        UniDistribution {
            n,
            generator: Rng::new()
        }
    }

    fn generate(&mut self) -> usize {
        self.generator.usize(1..=self.n)
    }
}

pub struct ArrDistribution {
    dist_vec: Vec<f64>,
    pub generator: Rng
}

impl ArrDistribution {
    fn new(dist_vec: Vec<f64>) -> Self {
        ArrDistribution {
            dist_vec,
            generator: Rng::new()
        }
    }

    pub fn generate(&mut self) -> usize {
        let r = self.generator.f64();
        self.dist_vec.iter().position(|&x| x > r).unwrap()
    }

    pub fn harmonic(max_int: usize) -> Self {
        let mut dist_vec: Vec<f64> = vec![0.0; max_int + 1];
        dist_vec[1] = 1.0;
        for i in 2..=max_int {
            dist_vec[i] = dist_vec[i - 1] + 1.0 / i as f64;
        }
        let h_max = dist_vec[max_int];
        let mut dist_vec: Vec<f64> =
            dist_vec.iter().map(|x| x / h_max).collect();
        dist_vec[max_int] = 1.0;
        Self::new(dist_vec)
    }

    pub fn biharmonic(max_int: usize) -> Self {
        let mut dist_vec: Vec<f64> = vec![0.0; max_int + 1];
        dist_vec[1] = 1.0;
        for i in 2..=max_int {
            dist_vec[i] = dist_vec[i - 1] + 1.0 / (i * i) as f64;
        }
        let h_max = dist_vec[max_int];
        let mut dist_vec: Vec<f64> =
            dist_vec.iter().map(|x| x / h_max).collect();
        dist_vec[max_int] = 1.0;
        Self::new(dist_vec)
    }
}

pub struct GeoDistribution {
    p : f64,
    n: usize,
    pub generator: Rng
}

impl GeoDistribution {
    pub fn new(p: f64, n: usize) -> Self {
        GeoDistribution {
            p,
            n,
            generator: Rng::new()
        }
    }

    pub fn generate(&mut self) -> usize {
        let mut count = 1;
        for _ in 1..self.n {
            if self.generator.f64() < self.p {
                return count
            }
            count += 1;
        }
        count
    }
}

pub enum Distribution {
    Uni(UniDistribution),
    Har(ArrDistribution),
    Bih(ArrDistribution),
    Geo(GeoDistribution)
}

impl Distribution {
    fn generate(&mut self) -> usize {
        match self {
            Distribution::Uni(dist) => dist.generate(),
            Distribution::Har(dist) => dist.generate(),
            Distribution::Bih(dist) => dist.generate(),
            Distribution::Geo(dist) => dist.generate()
        }
    }

    pub fn name(& self) -> String {
        match self {
            Distribution::Uni(_) => "uniform".to_owned(),
            Distribution::Har(_) => "harmonic".to_owned(),
            Distribution::Bih(_) => "biharmonic".to_owned(),
            Distribution::Geo(_) => "geometric".to_owned()
        }
    }

    pub fn gen_seq(&mut self, n: usize) -> Vec<f64> {
        let mut seq = Vec::with_capacity(n);
        let rand = Rng::new();

        while seq.len() < n {
            let r: f64 = rand.f64();
            let k = self.generate();
            for _ in 0..std::cmp::min(k, n - seq.len()) {
                seq.push(r);
            }
        }

        seq
    }
}
