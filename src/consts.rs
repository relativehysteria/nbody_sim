/// Maximum mass the bodies can spawn with
pub const MAX_MASS: f64 = 1000.;

/// Newtonian constant
pub const G: f64 = 1.;

/// Theta value for the Barnes-Hut method
pub const THETA: f64 = 0.5;

/// Amount of bodies to spawn
pub const N_BODIES: usize = 10_000;

/// Amount of black holes to spawn
pub const N_HOLES: usize = 1;

/// The maximum distance of the simulation in all directions
pub const MAX_DIST: usize = 1080;

/// The number of dimensions to simulate
pub const DIMENSIONS: usize = 2;

/// Time step in seconds
pub const DT: f64 = 1e-2;

/// For how many `DT` time steps the simulation runs before it stops
pub const SIM_STEPS: usize = usize::MAX;
