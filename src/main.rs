use std::f64::consts::PI;
use rand::{thread_rng, Rng};
use rand::distributions::{Uniform};

mod hypercube;

// test function to be optimized
fn rastrigin(input_vector: &Vec<f64>) -> f64 {
    let dimension = input_vector.len();
    let mut sum: f64 = 0.0;

    // iterating over slice of derefenced vector pointer
    for val in &*input_vector {
        sum += val.powf(2.0) - (10.0 * (2.0*PI*val).cos());
    }

    10.0 * dimension as f64 + sum
}

// used to generate a random point with a given dimension and upper and lower bounds
fn generate_random_point(dimension: i64, lower_bound: f64, upper_bound: f64) -> Vec<f64> {
    let mut rng = thread_rng();
    let uniform_range = Uniform::new_inclusive(lower_bound, upper_bound);
    let random_point: Vec<f64> = (&mut rng).sample_iter(uniform_range)
                                           .take(dimension.try_into().unwrap())
                                           .collect();
    random_point
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

    let rastrigin_val: f64 = rastrigin(&initial_point);
    println!("{}", rastrigin_val);
}
