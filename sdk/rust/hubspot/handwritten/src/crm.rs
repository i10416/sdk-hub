mod schema;
pub use schema::*;
pub mod contact;
pub mod prelude;
#[cfg(feature = "crm-v3")]
pub mod v3;
#[cfg(feature = "crm-v4")]
pub mod v4;
#[cfg(feature = "workflow")]
pub mod workflow;
