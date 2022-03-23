use crate::evaluation::PointEval;
use crate::hypercube::Hypercube;
use crate::point::Point;
use crate::result::HypercubeOptimizerResult;
use ordered_float::NotNan;
use std::collections::BinaryHeap;
use std::f32::consts::E;

pub struct HypercubeOptimizer {
    /// dimension of the optimization problem
    dimension: u32,
    init_point: Point,
    /// list of hypercubes created by the optimizer
    hypercube: Hypercube,
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

        Self {
            dimension: init_point.dim(),
            init_point,
            hypercube,
            tol_x,
            tol_f,
            max_eval,
            max_timeout,
            lower_bound,
            upper_bound,
            objective_function,
        }
    }

    pub fn maximize(&mut self) -> Option<PointEval> {
        let init_eval = PointEval::with_eval(self.init_point.clone(), self.objective_function);

        // should take max_eval into consideration and not evaluate the function more times than that

        // TODO: compute no. of allowed hypercube evaluations from (max_eval and number of points in hypercube)

        // keep running score of average image
        let mut average_f = init_eval.get_eval();

        let mut best_evaluations: BinaryHeap<PointEval> = BinaryHeap::new();

        // records absolute change in F to compare with tolF
        let mut abs_delta_f_vec = Vec::with_capacity(30);

        // start optimization loop and measure time

        println!(
            "initial hypercube size: {}\n",
            self.hypercube.diagonal_len()
        );
        println!(
            "initial hypercube population size: {}\n",
            self.hypercube.get_population_size()
        );

        let mut previous_best_eval = init_eval;

        // start loop:
        for i in 0..self.max_eval {
            println!("-------- loop {} of {} --------\n", i, self.max_eval);

            // <----- hypercube randomize ----->

            self.hypercube.randomize_pop();

            // <----- hypercube evaluation ----->

            self.hypercube.evaluate(self.objective_function);

            // get best eval from current round of hypercube evaluations
            let current_best_eval = self.hypercube.peek_best_value().unwrap();

            best_evaluations.push(current_best_eval.clone());

            // calculate new average
            average_f = average_f + ((current_best_eval.get_eval() - average_f) / ((i + 1) as f64));

            // if current best is worse than average best value skip hypercube displacement and shrink
            if current_best_eval.get_eval() < average_f || current_best_eval < previous_best_eval {
                // if current best is worse than average best value skip hypercube displacement and shrink
                println!("skipping displacement and reinitializing hypercube...\n");
                continue;
            } else {
                println!("current best eval: {:#?}", current_best_eval);
                println!("previous best eval: {:#?}", previous_best_eval);
            }

            // calculate difference between previous best and current best
            let abs_delta_f = (current_best_eval.get_eval() - previous_best_eval.get_eval()).abs();

            if abs_delta_f <= self.tol_f {
                abs_delta_f_vec.push(abs_delta_f);

                // if the delta_f is within the tolerance consecutively more than 30 times, break
                // optimization loop
                if abs_delta_f_vec.len() >= 30 {
                    println!("optimization process terminated due to image convergence");
                    break;
                } else {
                    abs_delta_f_vec.clear();
                }
            }

            // <----- hypercube displace preparation ----->

            // compute new hypercube center (will be the average of old and new best value)
            let temp = &current_best_eval.get_point() + &previous_best_eval.get_point();
            let new_hypercube_center = temp.scale(0.5);

            // <----- hypercube shrink preparation ----->

            // compute normalized values

            // X_n
            let previous_normalized = (&previous_best_eval.get_point()
                - self.hypercube.get_center())
            .scale(1.0 / self.hypercube.get_side_length());

            // X_min_n
            let current_normalized = (&current_best_eval.get_point() - self.hypercube.get_center())
                .scale(1.0 / self.hypercube.get_side_length());

            // compute normalized distance
            let normalized_sqr_diff = (&(&current_normalized - &previous_normalized)
                * &(&current_normalized - &previous_normalized));

            let sum_normalized_sqr_diff = normalized_sqr_diff.sum();

            let normalized_distance =
                sum_normalized_sqr_diff.powf(0.5) / self.hypercube.get_side_length();

            // compute renormalized distance
            let renormalized_distance = normalized_distance / ((self.dimension as f64).sqrt());

            // compute convergence factor
            let convergence_factor =
                HypercubeOptimizer::calculate_convergence(renormalized_distance);

            println!("{}", self.hypercube);

            println!(
                ">>> Previous eval point: {:?}",
                previous_best_eval.get_point()
            );
            println!(
                ">>> Current eval point: {:?}",
                current_best_eval.get_point()
            );
            println!(">>> Previous normalized: {:?}", previous_normalized);
            println!(">>> Current normalized: {:?}", current_normalized);
            println!(">>> Normalized distance: {}", normalized_distance);
            println!(">>> Renormalized distance: {}", renormalized_distance);
            println!(">>> Convergence factor: {}\n", convergence_factor);

            // <----- hypercube displace ----->

            println!("attempting displacement to {:#?}", new_hypercube_center);
            self.hypercube.displace_to(&new_hypercube_center);

            // <----- hypercube shrink ----->

            self.hypercube.shrink(convergence_factor);

            previous_best_eval = current_best_eval;

            // end loop:
        }

        println!("final hypercube size: {}\n", self.hypercube.diagonal_len());

        let res = best_evaluations.peek().unwrap().clone();

        res
    }

    fn calculate_convergence(renormalized_distance: f64) -> f64 {
        let s = 1.0 - (0.2 * E.powf((-3.0 * renormalized_distance) as f32));
        s as f64
    }
}
