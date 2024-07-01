/// Maximum mass the bodies can spawn with
pub const MAX_MASS: f64 = 1000.;

/// Maximum radius the bodies can spawn with
pub const MAX_RAD: f64 = 5.;

/// Newtonian constant
pub const G: f64 = 1.;

/// Theta value for the Barnes-Hut method
pub const THETA: f64 = 0.5;

/// Amount of bodies to spawn
pub const N_BODIES: usize = 2;

/// The maximum distance of the simulation in all directions
pub const MAX_DIST: usize = 1000;

/// The number of dimensions to simulate
pub const DIMENSIONS: usize = 2;

/// Time step in seconds
pub const DT: f64 = 1.;

/// For how many `DT` time steps the simulation runs before it stops
pub const SIM_STEPS: usize = 10;
