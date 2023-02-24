use rand::{thread_rng, Rng};

pub struct Distribution {
    dist_vec: Vec<f64>,
    pub generator: rand::rngs::ThreadRng
}

impl Distribution {
    fn new(dist_vec: Vec<f64>) -> Self {
        Distribution {
            dist_vec,
            generator: thread_rng()
        }
    }

    pub fn generate(&mut self) -> usize {
        let r = self.generator.gen_range(0.0..1.0);
        self.dist_vec.iter().position(|&x| x > r).unwrap()
    }

    pub fn uniform(max_int: usize) -> Self {
        let max_float = max_int as f64;
        let mut dist_vec: Vec<f64> =
            (0..=max_int)
            .map(|x| x as f64 / max_float)
            .collect();
        dist_vec[max_int] = 1.0;
        Self::new(dist_vec)
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

    pub fn geometric(max_int: usize) -> Self {
        let mut dist_vec: Vec<f64> = vec![0.0; max_int + 1];
        dist_vec[max_int] = 1.0;
        for i in (1..max_int).rev() {
            let two_pow_i: f64 = 2.0f64.powi(-1 * i as i32);
            for j in i..max_int {
                dist_vec[j] += two_pow_i;
            }
        }
        Self::new(dist_vec)
    }
}
