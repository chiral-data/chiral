mod dataset;
mod cu;

pub use dataset::Kind as Dataset;
pub use crate::apps::kind::Kind as Operator;
pub use crate::apps::kind::ComputationKind;
pub use cu::Kind as ComputingUnit;