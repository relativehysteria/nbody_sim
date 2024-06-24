use rayon::prelude::*;

use std::collections::HashSet;
use std::sync::{Mutex, RwLock};
use std::ops::{Mul, Add, Div};

use crate::consts::*;
use crate::body::Body;
use crate::vector::VecN;
use crate::quadtree::QuadTreeNode;

/// Theta value used to evaluate force.
/// approaching 0 = accurate calculations
/// approaching 1 = aggressive approximations
const FMM_THETA: f64 = 1.0;

fn merge_connected<const DIMENSIONS: usize>(bodies: &mut Vec<Body<DIMENSIONS>>) {
    let pairs_to_merge = Mutex::new(Vec::new());
    let body_count = bodies.len();

    // Find all pairs of bodies that should be merged
    (0..body_count).into_par_iter().for_each(|i| {
        let mut local_pairs = Vec::new();
        for j in (i + 1)..body_count {
            let distance = bodies[i].pos.distance(&bodies[j].pos);
            if distance < MERGE_THRESHOLD {
                local_pairs.push((i, j));
            }
        }
        pairs_to_merge.lock().unwrap().extend(local_pairs);
    });

    // Sort and merge bodies
    let mut pairs_to_merge = pairs_to_merge.into_inner().unwrap();
    pairs_to_merge.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_indices = HashSet::new();
    let mut new_bodies = Vec::new();

    for (i, j) in pairs_to_merge {
        let body1 = &bodies[i];
        let body2 = &bodies[j];

        if merged_indices.contains(&body1.id) ||
            merged_indices.contains(&body2.id) {
            continue;
        }

        // Merge bodies[i] and bodies[j]
        let new_mass = body1.mass + body2.mass;
        let new_pos = body1.pos.mul(body1.mass)
            .add(body2.pos.mul(body2.mass)).div(new_mass);
        let new_vel = body1.vel.mul(body1.mass)
            .add(body2.vel.mul(body2.mass)).div(new_mass);

        let new_body = Body {
            id: body1.id * -1,
            pos: new_pos,
            vel: new_vel,
            mass: new_mass * 0.6,
            force: RwLock::new(VecN::new([0.0; DIMENSIONS])),
        };

        merged_indices.insert(body1.id);
        merged_indices.insert(body2.id);
        new_bodies.push(new_body);
    }

    // Remove merged bodies and add new bodies
    bodies.retain(|body| !merged_indices.contains(&body.id));
    bodies.extend(new_bodies);
}


pub fn fmm<const DIMENSIONS: usize>(bodies: &mut Vec<Body<DIMENSIONS>>) {
    // Build the quadtree
    let mut quadtree: QuadTreeNode<DIMENSIONS> = QuadTreeNode::new(
        VecN::new([N_BODIES as f64 / 2.0; DIMENSIONS]), N_BODIES as f64);

    // Compute multipole expansion
    bodies.iter().for_each(|body| quadtree.insert(body.clone()));
    quadtree.compute_multipole_expansion();

    for _ in 1..=SIM_STEPS {
        // Update the forces
        bodies.par_iter().for_each(|body| {
            body.update_force_fmm(&quadtree, FMM_THETA);
        });

        // Update the bodies
        bodies.par_iter_mut().for_each(|body| {
            body.update_velocity(DT);
            body.update_position(DT);
        });

        // Compute multipole expansion
        quadtree.clear();
        bodies.iter().for_each(|body| quadtree.insert(body.clone()));
        quadtree.compute_multipole_expansion();

        // Merge bodies that are close enough
        merge_connected(bodies);
    }
}

pub fn naive<const DIMENSIONS: usize>(bodies: &mut Vec<Body<DIMENSIONS>>) {
    for _ in 1..=SIM_STEPS {
        // Update the forces
        bodies.par_iter().for_each(|body| {
            body.update_force(&bodies);
        });

        // Update the bodies
        bodies.par_iter_mut().for_each(|body| {
            body.update_velocity(DT);
            body.update_position(DT);
        });

        // Merge bodies that are close enough
        merge_connected(bodies);
    }
}
