pub mod closures;
pub mod concurrency;
pub mod ctrl_flow;
pub mod error_handling;
pub mod io;
pub mod iterators;
pub mod lifetimes;
pub mod ownership;
pub mod test_framework;
pub mod traits;
pub mod types;
pub mod variables;
pub mod unsafe_rust;

//Re-exports sub-modules for cleaner access in main.rs
//Functions in these modules are accessed via cncpt::collecitons::function() or similar
pub use ctrl_flow::ifs;
pub use ctrl_flow::loops;
pub use test_framework::function_tests;
pub use types::compound::collections;
pub use types::compound::strings;
pub use types::compound::structs;
pub use types::generics;
pub use types::primitives::scalar;
