#![allow(dead_code)]
use crate::vector::VecN;
use crate::consts::*;
use crate::quadtree::QuadTreeNode;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Body<const DIMENSIONS: usize> {
    pub id: isize,
    pub mass: f64,
    pub pos: VecN<DIMENSIONS>,
    pub vel: VecN<DIMENSIONS>,
    pub force: RwLock<VecN<DIMENSIONS>>,
}

impl<const DIMENSIONS: usize> core::clone::Clone for Body<DIMENSIONS> {
    fn clone(&self) -> Body<DIMENSIONS> {
        let body = Body::new(self.id, self.mass,
                                 self.pos.clone(), self.vel.clone());
        *body.force.write().unwrap() = *self.force.read().unwrap();
        body
    }
}

impl<const DIMENSIONS: usize> Body<DIMENSIONS> {
    /// Create a new body starting at `pos` with velocity `vel`
    pub fn new(id: isize, mass: f64, pos: VecN<DIMENSIONS>,
               vel: VecN<DIMENSIONS>) -> Self {
        Body {
            id,
            mass,
            pos,
            vel,
            force: RwLock::new(VecN::default()),
        }
    }

    /// Update the body's position from its velocity in a `dt` time step
    pub fn update_position(&mut self, dt: f64) {
        self.pos += self.vel * dt;
    }

    /// Update the body's velocity from the net forces acting on it in a `dt`
    /// time step
    pub fn update_velocity(&mut self, dt: f64) {
        let force = self.force.read().unwrap();
        self.vel += (*force / self.mass) * dt;
    }

    pub fn update_force(&self, bodies: &[Body<DIMENSIONS>]) {
        let mut net_force: VecN<DIMENSIONS> = VecN::default();

        for other in bodies {
            if self.id == other.id { continue; }

            let distance = self.pos.distance(&other.pos);
            let f_mag = G * self.mass * other.mass / (distance.powi(2) + EPSSQ);
            let mut f_dir = self.pos - other.pos;
            f_dir.normalize();
            net_force -= f_dir * f_mag;
        }

        {
            let mut force = self.force.write().unwrap();
            *force = net_force
        }
    }

    pub fn update_force_fmm(&self, quadtree: &QuadTreeNode<DIMENSIONS>, theta: f64) {
        let net_force = quadtree.evaluate_force(self, theta);
        *self.force.write().unwrap() = net_force;
    }
}
