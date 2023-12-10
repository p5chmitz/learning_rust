pub mod types;
pub mod ctrl_flow;
pub mod ownership;
pub mod struct_test;
pub mod variables;
pub mod functions;

//Re-exports sub-modules for cleaner access in main.rs
//Functions in these modules are accessed via cncpt::collecitons::function() or similar
pub use types::compound::collections;
pub use types::primitives::scalar;
pub use ctrl_flow::ifs;
pub use ctrl_flow::loops;

