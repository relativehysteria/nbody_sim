use std::arch::x86_64::_rdtsc;
use std::time::Instant;

use nbody_simulation::body::Body;
use nbody_simulation::vector::VecN;
use nbody_simulation::consts::*;
use nbody_simulation::rng::Rng;
use nbody_simulation::implementations::*;

fn random_body<const DIMENSIONS: usize>(rng: &mut Rng, id: isize,
                                        mass: (f64, f64)) -> Body<DIMENSIONS> {
    let mass = rng.range(mass.0 as u64, mass.1 as u64) as f64;
    let pos = VecN::new([rng.range(0, N_BODIES as u64 * 2) as f64; DIMENSIONS]);
    let vel = VecN::new([0.; DIMENSIONS]);

    Body::new(id, mass, pos, vel)
}

fn main() {
    // Initialize the RNG
    let seed = unsafe { _rdtsc() };
    println!("SEED: {seed}");
    let mut rng = Rng::new(seed);


    // Spawn the bodies
    let bodies: Vec<_> = (0..N_BODIES)
        .map(|id| random_body::<DIMENSIONS>(&mut rng, id as isize, (1., MAX_MASS)))
        .collect();

    let now = Instant::now();
    fmm(&mut (bodies.clone()));
    println!("{:?}", now.elapsed());

    let now = Instant::now();
    naive(&mut (bodies.clone()));
    println!("{:?}", now.elapsed());
}
