pub mod guessing_game;
pub mod json;
pub mod rng_range;
pub mod test;

//Re-exports sub-module function for cleaner access in main.rs
pub use guessing_game::guessing_game;
pub use guessing_game::guessing_game_2;
pub use guessing_game::guessing_game_3;
pub use json::json_parsing;
pub use rng_range::rng_range;
