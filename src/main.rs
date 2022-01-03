use crate::objective_functions::objective_functions::rastrigin;
use crate::point::Point;

mod hypercube;
mod objective_functions;
mod bounds;
mod point;

fn main() {
    // create HypercubeOptimizer object here with certain parameters
    // HypercubeOptimizer will create mutable Hypercube object and manipulate it within a loop
    // HypercubeOptimizer.run() should take an objective function and bounds

    let dimension: u32 = 3;
    let init_bounds = bounds::Bounds::new(0.0, 120.0);

    let mut cube = hypercube::Hypercube::new(dimension, init_bounds);

    cube.evaluate(rastrigin);

    let destination: Point = point![23.2, 12.2, 32.4];

    let displacement_result = cube.displace(&destination);

    match displacement_result {
        Ok(()) => println!("displacement successful"),
        Err(message) => println!("{}", message)
    }
}
