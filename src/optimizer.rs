use crate::evaluation::PointEval;
use crate::hypercube::Hypercube;
use crate::point::Point;
use crate::result::HypercubeOptimizerResult;
use std::collections::BinaryHeap;
use std::f32::consts::E;
use std::time::Instant;

/// Represents a hypercube optimizer
pub struct HypercubeOptimizer {
    /// dimension of the optimization problem
    dimension: u32,

    /// first point inside the search space to evaluate
    init_point: Point,

    /// hypercube used for optimization
    hypercube: Hypercube,

    /// desired tolerance for the difference between consecutive function inputs
    tol_x: f64,

    /// desired tolerance for the difference between consective function evaluations
    tol_f: f64,

    /// maximum number of optimization loops allowed
    max_loop: u32,

    /// maximum number of function evaluations allowed
    max_eval: u32,

    /// maximum amount of time to optimize objective function
    max_timeout: u32,

    /// lower bound of the search space
    lower_bound: f64,

    /// upper bound of the search space
    upper_bound: f64,
}

impl HypercubeOptimizer {
    /// Returns a new `HypercubeOptimizer`
    ///
    /// # Arguments
    ///
    /// * `init_point` - the initial point inside the optimization search space to evaluate
    /// * `lower_bound` - the lower bound of the initial hypercube that defines the search space
    /// * `upper_bound` - the upper bound of the initial hypercube that defines the search space
    /// * `tol_x` - once the delta between consecutive best objective function inputs falls below this
    /// value, the optimization process will terminate
    /// * `tol_f` - once the delta between consecutive best objective function outputs falls below
    /// this value, the optimization process will terminate
    /// * `max_loop` - the maximum number of times the optimization loop is allowed to run
    /// * `max_eval` - the maximum number of objective function evaluations the optimizer will
    /// execute
    /// * `max_timeout` - the maximum amount of time for the optimization process to run for
    ///
    pub fn new(
        init_point: Point,
        lower_bound: f64,
        upper_bound: f64,
        tol_x: f64,
        tol_f: f64,
        max_loop: u32,
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

        Self {
            dimension: init_point.dim(),
            init_point,
            hypercube,
            tol_x,
            tol_f,
            max_loop,
            max_eval,
            max_timeout,
            lower_bound,
            upper_bound,
        }
    }

    pub fn maximize<F>(&mut self, obj_function: F) -> HypercubeOptimizerResult
    where
        F: Fn(&Point) -> f64,
    {
        // <----- Optimization result set-up ----->

        let start_time = Instant::now();

        let fn_eval = 0;

        let init_eval = PointEval::with_eval(self.init_point.clone(), &obj_function);

        // TODO: compute no. of allowed hypercube evaluations from max_eval and number of points
        // in hypercube

        // keep track of average image
        let mut average_f = init_eval.get_eval();

        let mut best_evaluations: BinaryHeap<PointEval> = BinaryHeap::new();

        // records absolute change in F to compare with tolF
        let mut abs_delta_f_vec = Vec::with_capacity(30);

        log::info!("initial hypercube size: {}", self.hypercube.diagonal_len());
        log::info!(
            "initial hypercube population size: {}",
            self.hypercube.get_population_size()
        );

        let mut previous_best_eval = init_eval;

        // start optimization loop
        for i in 0..self.max_loop {
            // <----- hypercube randomize ----->

            self.hypercube.randomize_pop();

            // <----- hypercube evaluation ----->

            self.hypercube.evaluate(&obj_function);

            // get best eval from current hypercube evaluation
            let current_best_eval = self.hypercube.peek_best_value().unwrap();

            if current_best_eval > previous_best_eval {
                best_evaluations.push(current_best_eval.clone());
            } else {
                best_evaluations.push(previous_best_eval.clone())
            }

            // calculate difference between previous best and current best
            let abs_delta_f = (current_best_eval.get_eval() - previous_best_eval.get_eval()).abs();

            if abs_delta_f <= self.tol_f {
                abs_delta_f_vec.push(abs_delta_f);

                // if the delta_f is within the tolerance consecutively more than 30 times, break
                // optimization loop
                if abs_delta_f_vec.len() >= 30 {
                    log::warn!("optimization process terminated due to image convergence");
                    let best_value: Option<&PointEval> = best_evaluations.peek();

                    let time_elapsed = start_time.elapsed();

                    return HypercubeOptimizerResult::new(0, i, fn_eval, best_value, time_elapsed);
                }
            } else {
                abs_delta_f_vec.clear();
            }

            // calculate new average
            average_f = average_f + ((current_best_eval.get_eval() - average_f) / ((i + 1) as f64));

            // if current best is worse than average best value skip hypercube displacement and shrink
            if current_best_eval.get_eval() < average_f || current_best_eval < previous_best_eval {
                continue;
            } else {
                log::info!(
                    "--------------- loop {} of {} ---------------",
                    i,
                    self.max_loop
                );
                log::info!("current best eval: {}", current_best_eval);
                log::info!("previous best eval: {}", previous_best_eval);
            }

            // <----- hypercube displace preparation ----->

            // compute new hypercube center (will be the average of old and new best value)
            let temp = &current_best_eval.get_point() + &previous_best_eval.get_point();
            let new_hypercube_center = temp.scale(0.5);

            // <----- hypercube shrink preparation ----->

            // compute X_n
            let previous_normalized = (&previous_best_eval.get_point()
                - self.hypercube.get_center())
            .scale(1.0 / self.hypercube.get_side_length());

            // compute X_min_n
            let current_normalized = (&current_best_eval.get_point() - self.hypercube.get_center())
                .scale(1.0 / self.hypercube.get_side_length());

            // compute normalized distance
            let normalized_sqr_diff = &(&current_normalized - &previous_normalized)
                * &(&current_normalized - &previous_normalized);

            let sum_normalized_sqr_diff = normalized_sqr_diff.sum();

            let normalized_distance =
                sum_normalized_sqr_diff.powf(0.5) / self.hypercube.get_side_length();

            // compute renormalized distance
            let renormalized_distance = normalized_distance / ((self.dimension as f64).sqrt());

            // compute convergence factor
            let convergence_factor =
                HypercubeOptimizer::calculate_convergence(renormalized_distance);

            log::info!("hypercube convergence factor: {}", convergence_factor);

            // <----- hypercube shrink ----->

            let pre_shrink_size = self.hypercube.diagonal_len();

            self.hypercube.shrink(convergence_factor);

            let post_shrink_size = self.hypercube.diagonal_len();

            log::info!(
                "shrunk hypercube from {} => {}",
                pre_shrink_size,
                post_shrink_size
            );

            // <----- hypercube displace ----->

            log::trace!("attempting displacement to {:#?}", new_hypercube_center);
            self.hypercube.displace_to(&new_hypercube_center);

            log::trace!("new hypercube center is {:#?}", self.hypercube.get_center());

            previous_best_eval = current_best_eval;

            // end loop:
        }

        log::info!("final hypercube size: {}", self.hypercube.diagonal_len());

        let best_value: Option<&PointEval> = best_evaluations.peek();
        let time_elapsed  = start_time.elapsed();

        HypercubeOptimizerResult::new(0, self.max_loop, fn_eval, best_value, time_elapsed)
    }

    /// Calculates the factor by which to shrink the hypercube during optimization
    ///
    /// # Arguments
    ///
    /// * `renormalized_distance` - the distance between the previous best and current best points
    /// in the search space if they existed within a unit hypercube
    fn calculate_convergence(renormalized_distance: f64) -> f64 {
        let s = 1.0 - (0.2 * E.powf((-3.0 * renormalized_distance) as f32));
        s as f64
    }
}
