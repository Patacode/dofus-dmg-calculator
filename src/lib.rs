//! Utility functions to compute a Dofus spell damage estimation inflicted on
//! an enemy with or without resistances.

mod api;
mod impls;

pub use api::compute_dmg_estimation;
pub use api::compute_dmg_estimation_with_res;
