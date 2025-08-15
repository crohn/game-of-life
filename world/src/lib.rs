//! Conway's Game of Life simulation engine.
//!
//! This crate implements Conway's Game of Life core logic, which is responsible for the management
//! of the board and the simulation advancement, following the application of the rules.
//!
//! The main entry point is the [World] struct, which holds the state and provides methods to
//! interact with the simulation. 

pub use world::World;

mod world;

