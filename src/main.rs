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

    let dimension: i64 = 10;
    let lower_bound: f64 = 0.0;
    let upper_bound: f64 = 120.0;

    // generate random initial point
    let initial_point: Vec<f64> = generate_random_point(dimension, lower_bound, upper_bound);

    let cube = hypercube::Hypercube::new(dimension, initial_point.clone(), upper_bound, lower_bound);

    let rastrigin_val: f64 = rastrigin(&initial_point);
    println!("{}", rastrigin_val);
}
