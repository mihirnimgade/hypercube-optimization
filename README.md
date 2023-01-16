# Hypercube Optimization

## Overview

This is a Rust implementation of a multi-dimensional hypercube-based optimization algorithm based on _"Optimization of High-Dimensional Functions through Hypercube Evaluation"_ (https://www.hindawi.com/journals/cin/2015/967320/).

## Installation

Simply add the following line to your `Cargo.toml` file as a dependency:

```toml
hypercube-optimization = { git = "https://github.com/mihirnimgade/hypercube-optimization" }
```

Cargo should then automatically download and compile the package next time you build your source code.

## Usage

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

If you would like to ensure the `hypercube-optimization` package is running correctly, you can run the included unit and integration tests with:

```shell
cargo test -p hypercube-optimization
```

## Algorithm outline

TODO: finish section

## Benchmarks

TODO: add benchmarks for following objective functions

- Rastrigin function
- Sphere function
- Rosenbrock function
- Ackley function
- Griewank function

## Current features

- Global optimization over arbitrary n-dimensional search space for arbitrary Rust vector function
- Built-in time measurement for optimization process

## Planned features

- Multithreading in optimizer core
- Python frontend that calls Rust backend using PyO3
- Expanded optimization parameter customization
- Customization for maximum optimization runtime and function evaluations
- Built-in logging system
