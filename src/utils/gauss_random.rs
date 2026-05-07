use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;

pub struct GaussSeed{
    rng: StdRng,
    current_gauss: Option<f64>,
    next_gauss: Option<f64>,
}

impl GaussSeed{
    pub fn seed(seed: u64) -> Self{
        GaussSeed{
            rng: StdRng::seed_from_u64(seed),
            current_gauss: None,
            next_gauss: None,
        }
    }

    pub fn next_gaussian(&mut self) -> f64{
        // if next gauss != current gauss, return current gauss
        if let Some(next) = self.next_gauss {
            self.current_gauss = Some(next);
            self.next_gauss = None;

            return self.current_gauss.unwrap();
        }

        let u1: f64 = self.next_random();
        let u2: f64 = self.next_random();

        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).sin();

        self.current_gauss = Some(z0);
        self.next_gauss = Some(z1); 

        z0
    }

    fn next_random(&mut self) -> f64 {
        self.rng.random_range(0.0..1.0)
    }
}