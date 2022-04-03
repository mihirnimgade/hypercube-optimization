use hypercube_optimizer::objective_functions::{neg_rastrigin, rastrigin};
use hypercube_optimizer::optimizer::HypercubeOptimizer;
use hypercube_optimizer::point;
use hypercube_optimizer::point::Point;

use simple_logger::SimpleLogger;

// extern crate pretty_env_logger;
// #[macro_use] extern crate log;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    // Log levels
    //
    // error! - Highest
    // warn!
    // info!
    // debug!
    // trace! - Lowest

    let dimension = 8;
    let initial_point = point![60.0; dimension];
    let lower_bound = 0.0;
    let upper_bound = 120.0;

    let mut optimizer = HypercubeOptimizer::new(
        initial_point,
        lower_bound,
        upper_bound,
        neg_rastrigin,
        0.01,
        0.1,
        2000,
        5000,
        120,
    );

    let result = optimizer.maximize();

    match result {
        None => {
            log::error!("unable to determine final result")
        }
        Some(t) => {
            log::info!("final result: {}", t);
        }
    }
}
