use std::f64::consts::PI;
mod hypercube;

// test function to be optimized
fn rastrigin(input_vector: &[f64]) -> f64 {
    let dimension = input_vector.len();
    let mut sum: f64 = 0.0;

    for &val in input_vector {
        sum += val.powf(2.0) - (10.0 * (2.0*PI*val).cos());
    }

    10.0 * dimension as f64 + sum
}

fn main() {
    // create HypercubeOptimizer object here with certain parameters
    // HypercubeOptimizer will create mutable Hypercube object and manipulate it within a loop
    // HypercubeOptimizer.run() should take an objective function and bounds

    const DIMENSION: usize = 6;
    let initial_point: &[f64] = &[1.0, 2.0, 4.2, 4.32, 5.7, 6.6];
    let lower_bound: &[f64; DIMENSION] = &[0.0; DIMENSION];
    let upper_bound: &[f64; DIMENSION] = &[120.0; DIMENSION];

    let cube = hypercube::Hypercube::new(initial_point, upper_bound, lower_bound);

    println!("{}\n", cube);

    println!("Input point: {:?}", initial_point);
    println!("Rastrigin value: {}", rastrigin(initial_point));
}
