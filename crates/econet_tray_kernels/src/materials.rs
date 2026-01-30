use serde::{Deserialize, Serialize};

/// Single component in a tray material recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialComponent {
    /// Identifier for the material (e.g., "bagasse", "PLA", "coating-X").
    pub name: String,
    /// Mass fraction (0–1) of this component in the recipe.
    pub fraction: f64,
}

/// Full material mix for a recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialMix {
    pub id: String,
    pub components: Vec<MaterialComponent>,
}

impl MaterialMix {
    /// Normalizes component fractions so they sum to 1.
    pub fn normalized(mut self) -> Self {
        let sum: f64 = self.components.iter().map(|c| c.fraction).sum();
        if sum > 0.0 {
            for c in &mut self.components {
                c.fraction /= sum;
            }
        }
        self
    }
}
