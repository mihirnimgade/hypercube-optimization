# Hypercube Optimization

## Project overview

This is a Rust implementation of a multi-dimensional hypercube-based optimization algorithm based on "Optimization of High-Dimensional Functions through Hypercube Evaluation" (https://www.hindawi.com/journals/cin/2015/967320/).

## Installation

## Example usage

```Rust
use hypercube_optimizer::objective_functions::neg_rastrigin;
use hypercube_optimizer::optimizer::HypercubeOptimizer;
use hypercube_optimizer::point;
use hypercube_optimizer::result::HypercubeOptimizerResult;

fn main() {
    let dimension = 8;                              // dimensionality of problem
    let initial_point = point![60.0; dimension];    // initial optimization input guess
    let lower_bound = 0.0;                          // lower bound of search space
    let upper_bound = 120.0;                        // upper bound of search space

    let mut optimizer = HypercubeOptimizer::new(
        initial_point,
        lower_bound,
        upper_bound,
        0.01,         // input tolerance
        0.1,          // output tolerance
        2000,         // maximum allowed evaluation loops
        5000,         // maximum allowed objective function evaluations
        120,          // maximum allowed optimization time (in seconds)
    );

    let result: HypercubeOptimizerResult = optimizer.maximize(neg_rastrigin);
    println!("final result: {:#?}", result);
}
```

## Running the tests

## Algorithm outline

## Benchmarks

## Current features

## Planned features
