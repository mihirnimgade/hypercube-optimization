# Hypercube Optimization

Welcome to the Github repository for the `hypercube-optimization` Rust package!

## Table of contents

- [Overview](#overview)
- [Algorithm outline](#algorithm-outline)
- [Installation](#installation)
- [Usage](#usage)
- [Running the tests](#running-the-tests)
- [Benchmarks](#benchmarks)
- [Current features](#current-features)
- [Planned features](#planned-features)

## Overview

The hypercube optimization algorithm is designed to globally optimize vector functions where the problem dimensionality is particularly high. In more mathematical terms, these vector functions have the following specification:

$$f: S \rightarrow \mathbb{R}, S \subseteq \mathbb{R}^{n}, n \in \mathbb{Z}^{+}$$

In the above notation, $f$ is the objective function, $S$ is the search space, and $n$ is the dimensionality of the problem. Ideally, the goal of the hypercube optimization algorithm (or really any global optimization algorithm) is to find some $s \in S$ such that $f(s)$ is "optimum" for whatever definition of optimum (usually either maximum or minimum).

It is important to note that this algorithm knows nothing about the internals of how the function is calculated nor does it need **any gradient information** about the function (e.g., Jacobian, Hessian). Rather, the algorithm treats the objective function as a **black box**.

## Algorithm outline

This implementation is based on _"Optimization of High-Dimensional Functions through Hypercube Evaluation"_ by Rahib H. Abiyev and Mustafa Tunay (https://www.hindawi.com/journals/cin/2015/967320/).

The algorithm's basic process involves initializing an n-dimensional cube (hypercube) that is the size of your search space and randomly generating a population of points that reside within the hypercube. This population represents inputs to the objective function. The population is evaluated using the objective function and the current "best" point is acquired. 

Depending on the location of the current best point and the previous one, the hypercube displaces and shrinks in space to focus in on the global optimum.

A high-level flowchart is shown below:

![flowchart](/images/hypercube-flowchart.png)

## Installation

To use the package, simply add the following line to your `Cargo.toml` file as a dependency:

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

## Benchmarks

// TODO: add benchmarks for following objective functions:

- Rastrigin function
- Sphere function
- Rosenbrock function
- Ackley function
- Griewank function

## Current features

:heavy_check_mark: Global optimization over arbitrary n-dimensional search space for arbitrary Rust vector function

:heavy_check_mark: Built-in time measurement for optimization process

## Planned features

:rocket: Multi-threading in optimizer core

:rocket: Python frontend that calls Rust backend using PyO3
 
:rocket: Expanded optimization parameter customization

:rocket: Customization for maximum optimization runtime and function evaluations

:rocket: Built-in logging system
