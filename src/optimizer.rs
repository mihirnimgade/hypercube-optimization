use crate::hypercube::Hypercube;
use crate::result::HypercubeOptimizerResult;
use crate::Point;

pub struct HypercubeOptimizer {
    dimension: u32,
    init_point: Point,
    /// list of hypercubes created by the optimizer
    hypercubes: Vec<Hypercube>,
    /// minimum acceptable tolerance for the difference between X inputs between iterations
    tol_x: f64,
    tol_f: f64,
    /// maximum number of function evaluations allowed
    max_eval: u32,
    /// maximum amount of time to spend optimizing
    max_timeout: u32,
    lower_bound: f64,
    upper_bound: f64,
    objective_function: fn(&Point) -> f64,
}

impl HypercubeOptimizer {
    // take optimization parameters here
    pub fn new(
        init_point: Point,
        lower_bound: f64,
        upper_bound: f64,
        objective_function: fn(&Point) -> f64,
        tol_x: f64,
        tol_f: f64,
        max_eval: u32,
        max_timeout: u32,
    ) -> Self {
        assert!(
            upper_bound > lower_bound,
            "Upper bound not strictly larger than lower bound"
        );
        assert!(
            init_point.max_val().unwrap() <= upper_bound,
            "init_point not inside upper bound"
        );
        assert!(
            init_point.min_val().unwrap() >= lower_bound,
            "init_point not inside lower bound"
        );

        // create initial hypercube based on initial bounds and place inside vector
        let hypercube = Hypercube::new(init_point.dim(), lower_bound, upper_bound);

        // TODO: change this so that its different depending on certain factors
        let mut hypercube_vector: Vec<Hypercube> = Vec::with_capacity(256);
        hypercube_vector.push(hypercube);

        Self {
            dimension: init_point.dim(),
            init_point,
            hypercubes: hypercube_vector,
            tol_x,
            tol_f,
            max_eval,
            max_timeout,
            lower_bound,
            upper_bound,
            objective_function,
        }
    }

    pub fn optimize() -> HypercubeOptimizerResult {
        // evaluate initial point and store initial value

        // should take max_eval into consideration and not evaluate the function beyond that

        // compute hypercube evaluations from max_eval and number of points in hypercube

        // start optimization loop and measure time

        //

        todo!()
    }

    fn calculate_convergence(&self) -> f64 {
        todo!()
    }
}
