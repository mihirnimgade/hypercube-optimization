use hypercube_optimizer::objective_functions::{neg_rastrigin, rastrigin};
use hypercube_optimizer::optimizer::HypercubeOptimizer;
use hypercube_optimizer::point;
use hypercube_optimizer::point::Point;

fn main() {
    let dimension = 3;
    let initial_point = point![4.0; dimension];
    let lower_bound = -5.0;
    let upper_bound = 5.0;

    // create HypercubeOptimizer with certain parameters
    let mut optimizer = HypercubeOptimizer::new(
        initial_point,
        lower_bound,
        upper_bound,
        neg_rastrigin,
        0.01,
        0.01,
        200,
        120,
    );

    let result = optimizer.maximize();

    println!("Final result: {:?}", result);

    // HypercubeOptimizer.optimize() should not take any arguments and should return HypercubeOptimizerResult struct
}
