use hypercube_optimizer::objective_functions::{neg_rastrigin, rastrigin};
use hypercube_optimizer::optimizer::HypercubeOptimizer;
use hypercube_optimizer::point;
use hypercube_optimizer::point::Point;

fn main() {
    let dimension = 8;
    let initial_point = point![60.0; dimension];
    let lower_bound = 0.0;
    let upper_bound = 120.0;

    // create HypercubeOptimizer with certain parameters
    let mut optimizer = HypercubeOptimizer::new(
        initial_point,
        lower_bound,
        upper_bound,
        neg_rastrigin,
        0.01,
        0.01,
        4000,
        120,
    );

    let result = optimizer.maximize();

    match result {
        None => {
            println!("Unable to determine final result")
        }
        Some(t) => {
            println!("Final result: {:?}", t);
        }
    }
}
