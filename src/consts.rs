/// Maximum mass the bodies can spawn with
pub const MAX_MASS: f64 = 1000.;

/// Amount of bodies to spawn
pub const N_BODIES: usize = 30_000;

/// Gravitational constant
pub const G: f64 = 1e-8;

/// Value the prevents the force from becomin excessively large
pub const EPSILON: f64 = 0.5;

/// The distance threshold at which two bodies merge into one
pub const MERGE_THRESHOLD: f64 = 0.2;

/// Precalculated square of epsilon
pub const EPSSQ: f64 = EPSILON * EPSILON;

/// The number of dimensions to simulate
pub const DIMENSIONS: usize = 3;

/// Time step in seconds
pub const DT: f64 = 60. * 60.0;

/// For how many `DT` time steps the simulation runs before it stops
pub const SIM_STEPS: usize = 1024;
