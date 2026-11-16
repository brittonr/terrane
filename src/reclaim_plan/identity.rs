//! Verified finite reclaim planning over an indexed reference graph.
//!
//! The module checks and builds plans that retain everything reachable from a
//! root, remove only reclaimable nodes that no root reaches, and never leave a
//! surviving node referencing a removed one. It owns no store paths, plan
//! identities, byte accounting, or removal execution.
// r[impl reclaim-plan.validity]
// r[impl reclaim-plan.evidence]

pub mod verified;

pub use verified::ReclaimPlan;
pub use verified::admissible;
pub use verified::check_plan;
pub use verified::plan;
