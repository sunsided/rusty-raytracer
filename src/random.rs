use rand::{distributions::Uniform, prelude::*, thread_rng};

pub struct Random {
    rng: ThreadRng,
    distribution: Uniform<f64>,
}

impl Random {
    pub fn sample(&mut self) -> f64 {
        self.distribution.sample(&mut self.rng)
    }
}

impl Default for Random {
    fn default() -> Self {
        Self {
            rng: thread_rng(),
            distribution: Uniform::from(0.0f64..1.0),
        }
    }
}
