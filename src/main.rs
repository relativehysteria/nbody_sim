use nbody_simulation::body::Body;
use nbody_simulation::vector::VecN;

/// Gravitational constant
const G: f64 = 6.67430e-11;

/// The number of dimensions to simulate
const DIMENSIONS: usize = 2;

/// Time step in seconds
const DT: f64 = 60. * 60.;

/// For how many `DT` time steps the simulation runs before it stops
const SIM_STEPS: usize = usize::MAX;

/// The factor by which all vectors are scaled
const SCALE: f64 = 1.0e30;

/// The inverse of the scale to make calculations faster
const SCALE_INV: f64 = 1. / SCALE;

const EARTH_POS: VecN<DIMENSIONS> = VecN::new([1.496e11 * SCALE_INV, 0.]);
const EARTH_VEL: VecN<DIMENSIONS> = VecN::new([0., 29780. * SCALE_INV]);

fn update_forces<const DIMENSIONS: usize>(bodies: &mut [Body<DIMENSIONS>]) {
    for b1i in 0..bodies.len() {
        let mut force: VecN<DIMENSIONS> = VecN::default();

        for (b2i, b2) in bodies.iter().enumerate() {
            if b1i == b2i { continue; }
            let b1 = &bodies[b1i];

            let distance = b1.pos.distance(&b2.pos).powi(2) * 0.5;
            let f_mag = G * b1.mass * b2.mass * distance;
            let mut f_dir = b1.pos - b2.pos;
            f_dir.normalize();
            force += f_dir * f_mag;
        }

        bodies[b1i].force = force;
    }
}

fn check_nan<const DIMENSIONS: usize>(body: &Body<DIMENSIONS>) -> bool {
    body.pos.is_nan() || body.vel.is_nan() || body.force.is_nan()
}

fn main() {
    const NULL_VEC: VecN<DIMENSIONS> = VecN::new([0., 0.]);

    // Spawn the bodies
    let mut bodies = [
        Body::new("Sun", 1.989e30 * SCALE_INV, NULL_VEC, NULL_VEC),
        Body::new("Earth", 5.972e24 * SCALE_INV, EARTH_POS, EARTH_VEL),
        Body::new("Moon", 7.348e22 * SCALE_INV,
                  EARTH_POS + VecN::new([384_400_000. * SCALE_INV, 0.,]),
                  EARTH_VEL + VecN::new([0., 1022. * SCALE_INV])),
    ];

    for step in 1..=SIM_STEPS {
        // Update the forces
        update_forces(&mut bodies);

        // Update the bodies
        for body in bodies.iter_mut() {
            body.update_velocity(DT);
            body.update_position(DT);
            if check_nan(&body) {
                println!("{step}: {}", body.name);
                std::process::exit(1);
            }
        }

        // println!("Step: {step}: ---------------------------------------------");
        // for body in bodies.iter() {
        //     println!(">  {} | {} | {}", body.name, body.pos, body.vel);
        // }
    }
}
