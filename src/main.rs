use std::arch::x86_64::_rdtsc;

use nbody_simulation::{ Rng, SpatialTree, Body, VecN, BoundingBox };
use nbody_simulation::consts::*;

fn random_body<const DIMENSIONS: usize>(rng: &mut Rng, id: usize,
                                        rad: (f64, f64), mass: (f64, f64)
                                        ) -> Body<DIMENSIONS> {
    let mass = rng.range(mass.0 as u64, mass.1 as u64) as f64;
    let rad  = rng.range(rad.0 as u64, rad.1 as u64) as f64;
    let pos = VecN::new(
        core::array::from_fn(|_| rng.range(0, MAX_DIST as u64) as f64));
    let vel = VecN::new([0.; DIMENSIONS]);

    Body::new(id, mass, rad, pos, vel)
}

fn update_tree<const DIMENSIONS: usize>(tree: &mut SpatialTree<DIMENSIONS>,
                                        bodies: &[Body<DIMENSIONS>]) {
    let bounding_box: BoundingBox<DIMENSIONS> = BoundingBox::from(0., MAX_DIST as f64);
    *tree = SpatialTree::empty(bounding_box);
    for body in bodies {
        tree.insert(body.pos, body.mass, bounding_box);
    }
}

fn main() {
    // Initialize the RNG
    let seed = unsafe { _rdtsc() };
    println!("SEED: {seed}");
    let mut rng = Rng::new(seed);

    // Spawn the bodies
    let mut bodies: Vec<_> = (0..N_BODIES)
        .map(|id| random_body::<DIMENSIONS>(&mut rng, id,
                                            (1., MAX_RAD), (1., MAX_MASS)))
        .collect();

    // Crate the tree and insert the bodies
    let bounding_box: BoundingBox<DIMENSIONS> =
        BoundingBox::from(0., MAX_DIST as f64);
    let mut tree: SpatialTree<DIMENSIONS> = SpatialTree::empty(bounding_box);
    bodies.iter().for_each(|b| tree.insert(b.pos, b.mass, bounding_box));

    println!("{bodies:#?}");
    println!("{tree:#?}");

    for _ in 0..SIM_STEPS {
        // Update the tree with current body positions
        update_tree(&mut tree, &bodies);

        // Compute forces and update velocities
        for body in bodies.iter_mut() {
            let force = tree.compute_force(body, THETA);
            body.apply_force(force, DT);
        }

        // Update positions
        for body in bodies.iter_mut() {
            body.update_position(DT);
        }

        for body in bodies.iter() {
            print!("{} ", body.pos);
        }
        println!();
    }
}
