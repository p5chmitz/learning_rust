pub mod guessing_game;
pub mod rng_range;

//Re-exports sub-module function for cleaner access in main.rs
pub use guessing_game::guessing_game;
pub use guessing_game::guessing_game_2;
pub use rng_range::rng_range;
