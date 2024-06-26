use crate::{ VecN, BoundingBox, Body };
use crate::consts::*;

const DIM_POW: usize = 1 << DIMENSIONS;

#[derive(Clone, Debug)]
/// A spatial mass tree for `DIMENSIONS` dimensional simulators
pub struct SpatialTree<const DIMENSIONS: usize> {
    /// Position of the center of mass
    pub pos: VecN<DIMENSIONS>,

    /// Mass of this tree node
    pub mass: f64,

    /// Bounding box of this tree node
    pub bb: BoundingBox<DIMENSIONS>,

    /// Children of this tree node
    pub children: [Option<Box<Self>>; DIM_POW],
}

impl<const DIMENSIONS: usize> SpatialTree<DIMENSIONS> {
    /// Constucts a new tree with no children
    pub fn empty(bb: BoundingBox<DIMENSIONS>) -> Self {
        Self {
            pos: VecN::from(0.),
            mass: 0.,
            bb,
            children: core::array::from_fn(|_| None),
        }
    }

    /// Constructs a new child under the node `self.children[idx]`
    pub fn new_child(&mut self, idx: usize, pos: VecN<DIMENSIONS>, mass: f64,
                     bb: BoundingBox<DIMENSIONS>) {
        self.children[idx] = Some(Box::new(Self {
            pos,
            mass,
            bb,
            children: core::array::from_fn(|_| None),
        }))
    }

    /// Checks if this node is a leaf (has no children)
    pub fn is_leaf(&self) -> bool {
        self.children.iter().all(|i| i.is_none())
    }

    /// Update the center of mass of this node
    pub fn update_m_center(&mut self, pos: VecN<DIMENSIONS>, mass: f64) {
        // Calculate the new total mass of this node
        let new_mass    = self.mass + mass;
        let mut new_pos = VecN::from(0.);

        // Calculate the new center of this node
        for (idx, (p1, p2)) in self.pos.iter().zip(pos.iter()).enumerate() {
            new_pos[idx] = ((self.mass * p1) + (mass * p2)) / new_mass;
        }

        // Update
        self.pos = new_pos;
        self.mass = new_mass;
    }

    pub fn insert(&mut self, pos: VecN<DIMENSIONS>, mass: f64,
                  bb: BoundingBox<DIMENSIONS>) {
        // If inserting empty objects, return
        if mass <= 0. {
            return;
        }

        // If inserting the first element of the tree, update and return
        if self.mass <= 0. {
            self.pos = pos;
            self.mass = mass;
            self.bb = bb;
            return;
        }

        // Find the parent to insert this new node under
        let mut parent: &mut Self = self;
        let mut parent_bb = bb;
        let mut quadr = parent_bb.quadrant(pos);
        while let Some(_) = &mut parent.children[quadr] {
            // Update the parent center of mass
            parent.update_m_center(pos, mass);

            // Update the bounding box while searching for new parents
            parent_bb = parent_bb.child(quadr);
            parent = parent.children[quadr].as_mut().unwrap();

            // Compute the quadrant for next iteration
            quadr = parent_bb.quadrant(pos);
        }

        // We found a new parent into which we can fit this node.
        // If this new parent is a leaf, we must reinsert it into a deeper level
        // to maintain our tree constraints (one body per quadrant)
        if parent.is_leaf() {
            // Handle interactions if the bodies are too close
            const EPSILON: f64 = 1e-4;
            if parent.pos.distance(&pos) < EPSILON {
                // TODO: Low energy: Energy translation
                // TODO: Medium energy: Debris particles, energy translation

                // High energy interaction: Debris, merger of bodies
                // TODO: debris
                parent.update_m_center(pos, mass);
                return
            }

            // Calculate the center of mass between the two
            let (parent_pos, parent_mass) = (parent.pos, parent.mass);
            parent.update_m_center(pos, mass);
            let (child_pos, child_mass) = (parent.pos, parent.mass);

            // Then split until the parent and child are in separate cells
            let mut parent_quadr = parent_bb.quadrant(parent_pos);
            while quadr == parent_quadr {
                // Create the cell containing both
                parent.new_child(quadr, child_pos, child_mass,
                                 parent_bb.child(quadr));
                parent = parent.children[quadr].as_mut().unwrap();

                // Split at the center and continue down the tree
                parent_bb = parent_bb.child(quadr);
                quadr = parent_bb.quadrant(pos);
                parent_quadr = parent_bb.quadrant(parent_pos);
            }
            // Quadrants are different, insert the parent into its quadrant
            parent.new_child(parent_quadr, parent_pos, parent_mass,
                             parent_bb.child(parent_quadr));
        }
        // Insert the new child into its quadrant
        parent.new_child(quadr, pos, mass, parent_bb.child(quadr));
    }

    pub fn compute_force(&self, body: &Body<DIMENSIONS>,
                         theta: f64) -> VecN<DIMENSIONS> {
        // Calculate force using the Barnes-Hut algorithm
        let mut force = VecN::from(0.);

        if self.is_leaf() {
            // If it's a leaf node, calculate direct gravitational force
            let distance = self.pos.distance(&body.pos);
            if distance > 0.0 {
                let direction = (self.pos - body.pos) / distance;
                let force_magnitude = (G * self.mass * body.mass) / (distance * distance);
                force += direction * force_magnitude;
            }
        } else {
            // Otherwise, apply the Barnes-Hut criterion
            // This here assumes the size for all bounding boxes is uniform --
            // i.e. this uses the x coordinate
            let size = self.bb.diff(0);
            let distance = self.pos.distance(&body.pos);

            if size / distance < theta {
                // Treat this node as a single body
                let direction = (self.pos - body.pos) / distance;
                let force_magnitude = (G * self.mass * body.mass) / (distance * distance);
                force += direction * force_magnitude;
            } else {
                // Recursively calculate the force from each child
                for child in &self.children {
                    if let Some(child) = child {
                        force += child.compute_force(body, theta);
                    }
                }
            }
        }

        force
    }
}
