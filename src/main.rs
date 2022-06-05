use hypercube_optimizer::objective_functions::neg_rastrigin;
use hypercube_optimizer::optimizer::HypercubeOptimizer;
use hypercube_optimizer::point;
use hypercube_optimizer::point::Point;

use hypercube_optimizer::result::HypercubeOptimizerResult;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    let dimension = 8;
    let initial_point = point![60.0; dimension];
    let lower_bound = 0.0;
    let upper_bound = 120.0;

    let mut optimizer = HypercubeOptimizer::new(
        initial_point,
        lower_bound,
        upper_bound,
        0.01,
        0.1,
        2000,
        5000,
        120,
    );

    let result: HypercubeOptimizerResult = optimizer.maximize(neg_rastrigin);
    log::info!("final result: {:#?}", result);
}
