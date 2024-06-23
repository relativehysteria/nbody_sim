#![allow(dead_code)]
use crate::vector::VecN;

#[derive(Debug)]
pub struct Body<const DIMENSIONS: usize> {
    pub name: String,
    pub mass: f64,
    pub pos: VecN<DIMENSIONS>,
    pub vel: VecN<DIMENSIONS>,
    pub force: VecN<DIMENSIONS>,
}

impl<const DIMENSIONS: usize> Body<DIMENSIONS> {
    /// Create a new body starting at `pos` with velocity `vel`
    pub fn new(name: &str, mass: f64, pos: VecN<DIMENSIONS>,
               vel: VecN<DIMENSIONS>) -> Self {
        Body {
            name: name.to_string(),
            mass,
            pos,
            vel,
            force: VecN::default(),
        }
    }

    /// Update the body's position from its velocity in a `dt` time step
    pub fn update_position(&mut self, dt: f64) {
        self.pos += self.vel * dt;
    }

    /// Update the body's velocity from the net forces acting on it in a `dt`
    /// time step
    pub fn update_velocity(&mut self, dt: f64) {
        self.vel += (self.force / self.mass) * dt
    }
}
