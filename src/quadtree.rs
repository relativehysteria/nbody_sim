use core::cell::LazyCell;
use crate::vector::VecN;
use crate::body::Body;
use crate::consts::*;

const DIM_POW: usize = 1 << DIMENSIONS;

const MASKS: LazyCell<[usize; DIMENSIONS]> = LazyCell::new(|| {
    let mut m = [0; DIMENSIONS];
    for d in 0..DIMENSIONS {
        m[d] = 1 << d;
    }
    m
});

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
            bodies: Vec::with_capacity(N_BODIES),
            multipole_expansion: VecN::default(),
            children: None,
        }
    }

    pub fn clear(&mut self) {
        self.bodies.clear();

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                child.clear();
            }
        }
    }

    pub fn insert(&mut self, body: Body<DIMENSIONS>) {
        let mut current = self;

        loop {
            if current.size < 1.0 || current.bodies.len() < 4 {
                current.bodies.push(body);
                break;
            }

            if current.children.is_none() { current.subdivide(); }

            let idx = current.get_quadrant(&body.pos);
            if let Some(children) = current.children.as_mut() {
                current = &mut children[idx];
            } else {
                current.bodies.push(body);
                break;
            }
        }
    }

    pub fn subdivide(&mut self) {
        let half_size = self.size / 2.0;
        let mut centers = Vec::with_capacity(DIM_POW);
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
                index |= MASKS[d];
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

        if let Some(children) = &self.children {
            for child in children.iter() {
                net_force += child.evaluate_force(body, theta);
            }
        }

        if self.children.is_some() && !((self.size / distance) < theta) {
            return net_force;
        }

        let mass = self.bodies.iter().map(|b| b.mass).sum::<f64>();
        let f_mag = G * body.mass * mass / (distance.powi(2) + EPSSQ);
        let mut f_dir = body.pos - self.center;
        f_dir.normalize();
        net_force -= f_dir * f_mag;
        net_force
    }
}
