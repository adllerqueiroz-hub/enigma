pub mod pool;
pub mod request;
pub mod resolve;

pub use pool::{EntityDamageType, TargetContext, TargetEntity, TargetPool};
pub use request::TargetRequest;
pub use resolve::{TargetResolver, is_mapped_target_code, targets_enemy};
