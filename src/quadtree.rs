use crate::vector::VecN;
use crate::body::Body;
use crate::consts::*;

const DIM_POW: usize = 1 << DIMENSIONS;

#[derive(Default, Clone)]
pub struct QuadTreeNode<const DIMENSIONS: usize> {
    pub center: VecN<DIMENSIONS>,
    pub size: f64,
    pub bodies: Vec<Body<DIMENSIONS>>,
    pub multipole_expansion: VecN<DIMENSIONS>, // For simplicity, assume first-order expansion
    pub children: Option<Box<[QuadTreeNode<DIMENSIONS>; DIM_POW]>>,
}

impl<const DIMENSIONS: usize> QuadTreeNode<DIMENSIONS> {
    pub fn new(center: VecN<DIMENSIONS>, size: f64) -> Self {
        QuadTreeNode {
            center,
            size,
            bodies: Vec::new(),
            multipole_expansion: VecN::default(),
            children: None,
        }
    }

    pub fn insert(&mut self, body: Body<DIMENSIONS>) {
        if self.size < 1.0 || self.bodies.len() < 4 {
            self.bodies.push(body);
        } else {
            if self.children.is_none() {
                self.subdivide();
            }
            let idx = self.get_quadrant(&body.pos);
            if let Some(children) = self.children.as_mut() {
                children[idx].insert(body);
            }
        }
    }

    pub fn subdivide(&mut self) {
        let half_size = self.size / 2.0;
        let mut centers = Vec::new();
        for i in 0..DIM_POW {
            let mut offset = [0.0; DIMENSIONS];
            for d in 0..DIMENSIONS {
                let condition = ((i & (1 << d)) == 0) as usize as f64 * -1.;
                offset[d] = half_size * 0.5 * condition;
            }
            centers.push(self.center + VecN::new(offset));
        }

        let mut children: [QuadTreeNode<DIMENSIONS>; DIM_POW] =
            Default::default();
        for (i, center) in centers.into_iter().enumerate().take(DIM_POW) {
            children[i] = QuadTreeNode::new(center, half_size);
        }

        self.children = Some(Box::new(children));
    }

    pub fn get_quadrant(&self, pos: &VecN<DIMENSIONS>) -> usize {
        let mut index = 0;
        for d in 0..DIMENSIONS {
            if pos.coords[d] >= self.center.coords[d] {
                index |= 1 << d;
            }
        }
        index
    }

    pub fn compute_multipole_expansion(&mut self) {
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                child.compute_multipole_expansion();
            }
        }

        self.multipole_expansion = VecN::default();
        let mut total_mass = 0.0;

        for body in &self.bodies {
            total_mass += body.mass;
            self.multipole_expansion += body.pos * body.mass;
        }

        if total_mass > 0.0 {
            self.multipole_expansion /= total_mass;
        }
    }

    pub fn evaluate_force(&self, body: &Body<DIMENSIONS>,
                          theta: f64) -> VecN<DIMENSIONS> {
        let mut net_force = VecN::default();
        let distance = self.center.distance(&body.pos);

        if self.children.is_none() || (self.size / distance) < theta {
            let mass = self.bodies.iter().map(|b| b.mass).sum::<f64>();
            let f_mag = G * body.mass * mass / (distance.powi(2) + EPSSQ);
            let mut f_dir = body.pos - self.center;
            f_dir.normalize();
            net_force -= f_dir * f_mag;
        } else if let Some(children) = &self.children {
            for child in children.iter() {
                net_force += child.evaluate_force(body, theta);
            }
        }

        net_force
    }
}
