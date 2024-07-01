#![allow(dead_code)]
use crate::vector::VecN;

#[derive(Copy, Clone, Debug)]
/// A body (planet, asteroid, star, black hole etc.)
pub struct Body<const DIMENSIONS: usize> {
    /// Identifier of this body
    pub id: usize,

    /// Mass of the body
    pub mass: f64,

    /// Radius of the body
    pub rad: f64,

    /// Position of the body
    pub pos: VecN<DIMENSIONS>,

    /// Velocity of the body
    pub vel: VecN<DIMENSIONS>,
}

impl<const DIMENSIONS: usize> Body<DIMENSIONS> {
    /// Create a new body starting at `pos` with velocity `vel`
    pub fn new(id: usize, mass: f64, rad: f64, pos: VecN<DIMENSIONS>,
               vel: VecN<DIMENSIONS>) -> Self {
        Body {
            id,
            mass,
            rad,
            pos,
            vel,
        }
    }

    /// Update the body's position from its velocity in a `dt` time step
    pub fn update_position(&mut self, dt: f64) {
        self.pos += self.vel * dt;
    }
}
