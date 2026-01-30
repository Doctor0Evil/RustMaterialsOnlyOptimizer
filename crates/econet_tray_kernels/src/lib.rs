pub mod region;
pub mod materials;
pub mod kernels;
pub mod admissibility;

pub use region::RegionConfig;
pub use materials::{MaterialComponent, MaterialMix};
pub use kernels::{KernelOutputs, EvaluationError};
pub use admissibility::{admissible, Admissibility};
