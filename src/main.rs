use crate::objective_functions::objective_functions::rastrigin;
use crate::optimizer::HypercubeOptimizer;
use crate::point::Point;

mod bounds;
mod evaluation;
mod hypercube;
mod objective_functions;
mod optimizer;
mod point;
mod result;

fn main() {
    let dimension = 8;
    let initial_point = point![60.0; dimension];
    let lower_bound = 0.0;
    let upper_bound = 120.0;

    // create HypercubeOptimizer with certain parameters
    let optimizer = HypercubeOptimizer::new(
        initial_point,
        lower_bound,
        upper_bound,
        rastrigin,
        0.01,
        0.01,
        4000,
        120,
    );

    // HypercubeOptimizer.optimize() should not take any arguments and should return HypercubeOptimizerResult struct
}
