# Rust Materials-Only Optimizer

This repository implements a **materials-only** optimizer service that:

- Sweeps Phoenix tray recipe space.
- Evaluates each candidate using audited `econet_tray_kernels` risk kernels.
- Applies hard Phoenix gates on compostability and eco-risk.
- Writes only admissible recipes to the read-only shard:
  `qpudatashards/particles/BioPackPhoenixTrays2026v1.csv`.

It is designed to be a single, auditable source of truth for tray material
corridors in Phoenix EcoNet deployments.[web:0]

## Crates

- `econet_tray_kernels` – materials kernel crate (risk math and gates).
- `econet_materials_optimizer` – CLI/daemon that sweeps recipes and writes the shard.

External AI or orchestration systems **must** treat the CSV shard as
read-only evidence and must not recompute corridors or override flags.[web:0]
