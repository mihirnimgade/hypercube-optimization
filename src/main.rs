use crate::objective_functions::objective_functions::rastrigin;
use crate::point::Point;

mod bounds;
mod hypercube;
mod objective_functions;
mod optimizer;
mod point;
mod result;

fn main() {
    // create HypercubeOptimizer object here with certain parameters
    // HypercubeOptimizer will create mutable Hypercube object and manipulate it within a loop
    // HypercubeOptimizer.run() should take an objective function and bounds

    let dimension: u32 = 8;

    let mut cube = hypercube::Hypercube::new(dimension, 0.0, 120.0);

    let s = "-".repeat(20);

    println!("{} pre-evaluation and pre-shrink {}\n", s, s);
    println!("{}", cube);

    cube.evaluate(rastrigin);

    println!("Best value: {}", cube.peek_best_value().unwrap());

    // shrink and displace
    cube.shrink(0.01);
    let result = cube.displace_by(&point![1.0; dimension]);

    match result {
        Ok(()) => println!("displacement successful!\n"),
        Err(str) => println!("{}\n", str),
    }

    cube.evaluate(rastrigin);

    println!("{} post-evaluation and post-shrink {}\n", s, s);
    println!("{}", cube);
    println!("Best value: {}", cube.peek_best_value().unwrap());
}
