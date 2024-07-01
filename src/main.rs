use std::arch::x86_64::_rdtsc;

use macroquad::prelude::*;
use rayon::prelude::*;

use nbody_simulation::{ Rng, SpatialTree, Body, VecN, BoundingBox };
use nbody_simulation::consts::*;

fn random_body<const DIMENSIONS: usize>(rng: &mut Rng, id: usize,
                                        mass: (f64, f64)) -> Body<DIMENSIONS> {
    let m = rng.range(mass.0 as u64, mass.1 as u64) as f64;
    let rad  = m / mass.1 * 5.;
    let pos = VecN::new(
        core::array::from_fn(|_| rng.range(0, MAX_DIST as u64) as f64));
    let vel = VecN::new([0.; DIMENSIONS]);

    Body::new(id, m, rad, pos, vel)
}

fn update_tree<const DIMENSIONS: usize>(tree: &mut SpatialTree<DIMENSIONS>,
                                        bodies: &[Body<DIMENSIONS>]) {
    let bounding_box: BoundingBox<DIMENSIONS> = BoundingBox::from(0., MAX_DIST as f64);
    *tree = SpatialTree::empty(bounding_box);
    for body in bodies {
        tree.insert(body.pos, body.mass, bounding_box);
    }
}

fn render_bodies<const DIMENSIONS: usize>(bodies: &[Body<DIMENSIONS>]) {
    clear_background(BLACK);
    for body in bodies {
        let x = body.pos[0] as f32;
        let y = body.pos[1] as f32;
        let r = body.rad as f32;
        draw_circle_lines(x, y, r, 1., WHITE)
    }
}

#[macroquad::main("barnes")]
async fn main() {
    // Initialize the RNG
    let seed = unsafe { _rdtsc() };
    println!("SEED: {seed}");
    let mut rng = Rng::new(seed);

    // Spawn the bodies
    let mut bodies: Vec<_> = (0..N_BODIES)
        .map(|id| random_body::<DIMENSIONS>(&mut rng, id, (1., MAX_MASS)))
        .collect();

    // Crate the tree and insert the bodies
    let bounding_box: BoundingBox<DIMENSIONS> =
        BoundingBox::from(0., MAX_DIST as f64);
    let mut tree: SpatialTree<DIMENSIONS> = SpatialTree::empty(bounding_box);
    bodies.iter().for_each(|b| tree.insert(b.pos, b.mass, bounding_box));

    for _ in 0..SIM_STEPS {
        // Make sure bodies stay in bounds
        bodies.par_iter_mut().for_each(|body| {
            let limited = body.pos.limit(0., MAX_DIST as f64);
            if limited { body.vel.clear(); }
        });

        // Update the tree with current body positions
        update_tree(&mut tree, &bodies);

        // Compute forces and update velocities
        bodies.par_iter_mut().for_each(|body| {
            let force = tree.compute_force(body, THETA);
            body.apply_force(force, DT);
        });

        // Update positions
        bodies.par_iter_mut().for_each(|body| {
            body.update_position(DT);
        });

        // Render the bodies
        render_bodies(&bodies);
        next_frame().await;
    }
}
