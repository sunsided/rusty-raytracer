use rand::{distributions::Uniform, prelude::*, thread_rng};

pub struct Random {
    distribution: Uniform<f64>,
}

impl Random {
    pub fn sample(&self) -> f64 {
        let mut rng = thread_rng();
        self.distribution.sample(&mut rng)
    }
}

impl Default for Random {
    fn default() -> Self {
        Self {
            distribution: Uniform::from(0.0f64..1.0),
        }
    }
}
