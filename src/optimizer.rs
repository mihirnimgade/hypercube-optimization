use crate::evaluation::PointEval;
use crate::hypercube::Hypercube;
use crate::point::Point;
use crate::result::HypercubeOptimizerResult;
use ordered_float::NotNan;
use std::collections::BinaryHeap;

pub struct HypercubeOptimizer {
    /// dimension of the optimization problem
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

    pub fn maximize(&mut self) -> PointEval {
        // evaluate initial point and store value inside special struct
        let init_eval = PointEval::new_with_eval(self.init_point.clone(), self.objective_function);
        let mut previous_best_eval = init_eval;

        // should take max_eval into consideration and not evaluate the function more times than that

        // TODO: compute no. of allowed hypercube evaluations from (max_eval and number of points in hypercube)
        let max_eval = 1000;
        let max_hypercube_eval = 40;

        // keep running score of average image
        let mut average_f = 0.0;

        // records absolute change in F to compare with tolF
        let mut abs_delta_f_vec = Vec::with_capacity(30);

        // start optimization loop and measure time

        // retrieve first hypercube
        // let first_hypercube = self.hypercubes.first_mut().unwrap();
        println!(
            "initial hypercube size: {}\n",
            self.hypercube.diagonal_len()
        );
        println!(
            "initial hypercube population size: {}\n",
            self.hypercube.get_population_size()
        );

        // start loop:
        for i in 0..max_hypercube_eval {
            println!("Loop {} of {}", i, max_hypercube_eval);

            // <----- hypercube evaluation ----->

            first_hypercube.evaluate(self.objective_function);

            // get best eval
            let current_best_eval = first_hypercube.peek_best_value().unwrap();

            // compare to previous image and argument (will be PointEval struct)
            if current_best_eval.get_eval() <= average_f || current_best_eval <= previous_best_eval
            {
                // if current best is worse than average best value skip iteration
                // would want to reinitialize hypercube from here
                // do not displace hypercube
                println!("skipping displacement and reinitializing hypercube...\n");
                continue;
            } else {
                println!("current best eval: {:#?}", current_best_eval);
                println!("previous best eval: {:#?}", previous_best_eval);
            }

            // calculate difference between previous best and current best
            let abs_delta_f = (current_best_eval.get_eval() - previous_best_eval.get_eval()).abs();

            println!("Current best eval {:?}", current_best_eval);
            println!("Previous best eval {:?}", previous_best_eval);

            // TODO: can definitely write this better
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

            // calculate new average
            average_f = average_f + (current_best_eval.get_eval() - average_f) / ((i + 1) as f64);

            // compute new hypercube center (will be the average of old and new best value)
            let temp = &current_best_eval.get_point() + &previous_best_eval.get_point();
            let new_hypercube_center = temp.scale(0.5);

            // <----- hypercube displacement ----->

            let displacement_result = first_hypercube.displace_to(&new_hypercube_center);

            match displacement_result {
                Ok(_) => (),
                Err(s) => panic!("{}", s),
            }

            // <----- hypercube shrink ----->

            // compute renormalised distance

            // compute convergence factor

            // shrink hypercube

            // TODO: fix constant shrinking
            first_hypercube.shrink(0.80);

            // reset current best, set previous best equal to current best
            previous_best_eval = current_best_eval;

            // end loop:
        }

        // return result struct

        println!("final hypercube size: {}\n", self.hypercube.diagonal_len());

        best_evaluations.peek().unwrap().clone()
    }

    fn calculate_convergence(&self) -> f64 {
        todo!()
    }
}
