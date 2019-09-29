use rand;
use rand::distributions::Bernoulli;
use rand::distributions::Distribution;

struct Reality {
    distribution: Bernoulli,
}

impl Reality {
    fn new(p: f64) -> Self {
        Self {
            distribution: Bernoulli::new(p).unwrap(),
        }
    }

    fn sample(&self) -> bool {
        self.distribution.sample(&mut rand::thread_rng())
    }
}

fn main() {
    println!("Hello Bernoulli world! {:?}", Reality::new(0.5).sample());
}
