use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Mutex, RwLock};
use std::ops::{Mul, Add, Div};
use std::arch::x86_64::_rdtsc;

use nbody_simulation::body::Body;
use nbody_simulation::vector::VecN;
use nbody_simulation::consts::*;
use nbody_simulation::rng::Rng;

fn random_body<const DIMENSIONS: usize>(rng: &mut Rng, id: isize,
                                        mass: (f64, f64)) -> Body<DIMENSIONS> {
    let mass = rng.range(mass.0 as u64, mass.1 as u64) as f64;
    let pos = VecN::new([rng.range(0, N_BODIES as u64) as f64; DIMENSIONS]);
    let vel = VecN::new([0.; DIMENSIONS]);

    Body::new(id, mass, pos, vel)
}

fn merge_connected<const DIMENSIONS: usize>(bodies: &mut Vec<Body<DIMENSIONS>>,
                                            threshold: f64) {
    let pairs_to_merge = Mutex::new(Vec::new());
    let body_count = bodies.len();

    // Step 1: Find all pairs of bodies that should be merged
    (0..body_count).into_par_iter().for_each(|i| {
        let mut local_pairs = Vec::new();
        for j in (i + 1)..body_count {
            let distance = bodies[i].pos.distance(&bodies[j].pos);
            if distance < threshold {
                local_pairs.push((i, j));
            }
        }
        pairs_to_merge.lock().unwrap().extend(local_pairs);
    });

    // Step 2: Sort and merge bodies
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

    // Step 3: Remove merged bodies and add new bodies
    bodies.retain(|body| !merged_indices.contains(&body.id));
    bodies.extend(new_bodies);
}


fn main() {
    // Initialize the RNG
    let seed = unsafe { _rdtsc() };
    println!("SEED: {seed}");
    let mut rng = Rng::new(seed);

    // Keep a vector to prevent the simulation from devolving;
    // if within 10 steps there's no body merges, reset the simulation
    let mut last = Vec::with_capacity(10);

    loop {
        println!("Rng state: {}", rng.rand());

        // Spawn the bodies
        let mut bodies: Vec<_> = (0..N_BODIES)
            .map(|id| random_body::<DIMENSIONS>(&mut rng, id as isize, (10., 100.)))
            .collect();

        last.clear();

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
            merge_connected(&mut bodies, 0.2);

            // God has forsaken this place
            println!("{}", bodies.len());
            last.push(bodies.len());
            if last.len() == 10 {
                if last.iter().sum::<usize>() / 10 == bodies.len() { break; }
                last.clear();
            }
        }
    }
}
