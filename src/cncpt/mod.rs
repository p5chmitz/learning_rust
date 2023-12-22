pub mod ctrl_flow;
pub mod functions;
pub mod ownership;
pub mod types;
pub mod variables;
pub mod error_handling;

//Re-exports sub-modules for cleaner access in main.rs
//Functions in these modules are accessed via cncpt::collecitons::function() or similar
pub use ctrl_flow::ifs;
pub use ctrl_flow::loops;
pub use types::compound::collections;
pub use types::compound::strings;
pub use types::compound::structs;
pub use types::primitives::scalar;
