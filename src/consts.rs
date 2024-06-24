/// Amount of bodies to spawn
pub const N_BODIES: usize = 10_000;

/// Gravitational constant
pub const G: f64 = 1e-8;

/// Value the prevents the force from becomin excessively large
pub const EPSILON: f64 = 10.;

/// Precalculated square of epsilon
pub const EPSSQ: f64 = EPSILON * EPSILON;

/// The number of dimensions to simulate
pub const DIMENSIONS: usize = 2;

/// Time step in seconds
pub const DT: f64 = 60. * 10.;

/// For how many `DT` time steps the simulation runs before it stops
pub const SIM_STEPS: usize = usize::MAX;
