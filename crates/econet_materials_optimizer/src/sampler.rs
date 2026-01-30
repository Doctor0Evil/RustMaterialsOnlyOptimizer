use econet_tray_kernels::materials::{MaterialComponent, MaterialMix};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Simple sampler over a Phoenix tray recipe space.
///
/// In production, this is replaced with structured EDSK/MRK/ERLK recipe spaces.[web:0]
pub struct RecipeSampler {
    rng: StdRng,
    fraction_dist: Uniform<f64>,
}

impl RecipeSampler {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            fraction_dist: Uniform::new(0.01, 0.99),
        }
    }

    pub fn sample_recipe(&mut self, id_prefix: &str, index: usize) -> MaterialMix {
        let mut frac1 = self.fraction_dist.sample(&mut self.rng);
        let mut frac2 = self.fraction_dist.sample(&mut self.rng);
        let mut frac3 = self.fraction_dist.sample(&mut self.rng);

        let sum = frac1 + frac2 + frac3;
        frac1 /= sum;
        frac2 /= sum;
        frac3 /= sum;

        let components = vec![
            MaterialComponent { name: "bagasse".into(), fraction: frac1 },
            MaterialComponent { name: "starch".into(), fraction: frac2 },
            MaterialComponent { name: "PLA".into(), fraction: frac3 },
        ];

        MaterialMix {
            id: format!("{id_prefix}_{}", index),
            components,
        }
    }
}
