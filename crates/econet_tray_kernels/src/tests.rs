#[cfg(test)]
mod tests {
    use crate::admissibility::{admissible, Admissibility};
    use crate::kernels::eval_recipe;
    use crate::materials::{MaterialComponent, MaterialMix};
    use crate::region::RegionConfig;

    #[test]
    fn phoenix_default_admits_reasonable_recipe() {
        let region = RegionConfig::phoenix_default();
        let mix = MaterialMix {
            id: "phoenix_bagasse_v1".to_string(),
            components: vec![
                MaterialComponent { name: "bagasse".into(), fraction: 0.7 },
                MaterialComponent { name: "starch".into(), fraction: 0.3 },
            ],
        };

        let outputs = eval_recipe(&region, &mix).expect("eval should succeed");
        let status = admissible(&region, &outputs);

        assert_eq!(status, Admissibility::Admissible);
        assert!(outputs.t90_days <= region.target_t90_max_days);
        assert!(outputs.r_tox <= 0.10);
        assert!(outputs.r_micro <= 0.05);
        assert!(outputs.r_worm <= 0.10);
        assert!(outputs.r_bee <= 0.10);
        assert!(outputs.r_residual <= region.r_max);
    }

    #[test]
    fn rejects_extreme_risk_recipe() {
        let region = RegionConfig::phoenix_default();
        let mix = MaterialMix {
            id: "hard_coating_heavy".to_string(),
            components: vec![
                MaterialComponent { name: "coating-hard".into(), fraction: 0.9 },
                MaterialComponent { name: "PLA".into(), fraction: 0.1 },
            ],
        };

        let outputs = eval_recipe(&region, &mix).expect("eval should succeed");
        let status = admissible(&region, &outputs);

        assert_eq!(status, Admissibility::Rejected);
    }
}
