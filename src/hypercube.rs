use std::collections::BinaryHeap;
use std::fmt;

use crate::bounds::{BoundType, HypercubeBounds};
use crate::evaluation::PointEval;
use crate::point;
use crate::point::Point;
use ordered_float::NotNan;

use crate::bounds::BoundType::LowerBound;
use crate::bounds::BoundsOverlap;

#[derive(Clone)]
pub struct Hypercube {
    dimension: u32,
    init_bounds: HypercubeBounds,
    current_bounds: HypercubeBounds,
    diagonal: Point,
    center: Point,
    population_size: u64,
    population: Vec<Point>,
    values: Vec<PointEval>,
    ordered_values: BinaryHeap<PointEval>,
}

impl Hypercube {
    /// Creates a new hypercube with a given `dimension` and bounds.
    pub fn new(dimension: u32, lower_bound: f64, upper_bound: f64) -> Self {
        assert_ne!(dimension, 0, "dimension cannot be zero");
        assert!(
            upper_bound > lower_bound,
            "upper bound is not strictly larger than lower bound"
        );

        // generate initial bounds struct
        let init_bounds: HypercubeBounds =
            HypercubeBounds::new(dimension, lower_bound, upper_bound);

        // TODO: replace with function that takes dimension and bounds and returns number of hypercube points
        let num_points = dimension.pow(2) * ((upper_bound - lower_bound) as u32);

        // calculate the hypercube's diagonal
        let hypercube_diagonal: Point =
            &point![upper_bound; dimension] - &point![lower_bound; dimension];

        let random_points = Hypercube::generate_random_points(
            dimension,
            num_points as u64,
            lower_bound,
            upper_bound,
        );

        let population_size = random_points.len() as u64;

        // generate center vector
        let central_value: f64 = (upper_bound + lower_bound) / 2.0;
        let center: Point = point![central_value; dimension];

        // return Hypercube struct
        Self {
            dimension,
            init_bounds: init_bounds.clone(),
            current_bounds: init_bounds.clone(),
            diagonal: hypercube_diagonal,
            center,
            population_size,
            population: random_points,
            values: Vec::with_capacity(population_size as usize),
            ordered_values: BinaryHeap::with_capacity(population_size as usize),
        }
    }

    /// Applies the vector function to all points in the population and stores it in the hypercube
    /// struct.
    pub fn evaluate(&mut self, point_function: fn(&Point) -> f64) {
        // iterate over population points, apply vector function, and store result in values and
        // ordered_values
        for point in &self.population {
            // TODO: improve this so unnecessary cloning is removed
            let new_eval = PointEval::with_eval(point.clone(), point_function);
            self.values.push(new_eval.clone());
            self.ordered_values.push(new_eval);
        }
    }

    /// Peek at the maximum value evaluated by the hypercube
    pub fn peek_best_value(&self) -> Option<PointEval> {
        match self.ordered_values.peek() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    /// Pop the maximum value evaluated by the hypercube
    pub fn pop_best_value(&mut self) -> Option<PointEval> {
        match self.ordered_values.pop() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    /// Displaces the hypercube by adding the `vector` argument to the hypercube's center.
    pub fn try_displace_by(&mut self, vector: &Point) -> Result<(), &'static str> {
        // ensures the destination vector is the correct dimension
        assert_eq!(
            vector.dim() as u32,
            self.dimension,
            "vector is not the correct size. \
            expected {}, got {}.",
            self.dimension,
            vector.dim()
        );

        // test adding destination vector to current bounds
        let new_bounds: HypercubeBounds = self.current_bounds.displace_by(vector);

        // if new bounds are within the bounds that the hypercube was initialized with
        match new_bounds.within(&self.init_bounds) {
            BoundsOverlap::NoneOutOfBounds => {
                // add vector to all points in population
                for point in self.population.iter_mut() {
                    *point += vector.clone();
                }

                // current bounds should now be new_bounds
                self.current_bounds = new_bounds;

                // alter center value
                self.center += vector.clone();

                // wipe out previous evaluation results
                self.values.clear();
                self.ordered_values.clear();

                // calculate new diagonal
                self.diagonal = self.current_bounds.get_upper() - self.current_bounds.get_lower();

                Ok(())
            }
            _ => Err("cannot displace, displacement results in hypercube out of bounds"),
        }
    }

    /// Displaces the hypercube by moving the center to the `destination` argument.
    pub fn try_displace_to(&mut self, destination: &Point) -> Result<(), &'static str> {
        // TODO: should make sure destination is not outside of initial bounds

        // ensures the destination vector is the correct dimension
        assert_eq!(
            destination.dim() as u32,
            self.dimension,
            "destination is not the correct dimension. \
            expected {}, got {}.",
            self.dimension,
            destination.dim()
        );

        let center_to_destination = destination - &self.center;

        self.try_displace_by(&center_to_destination)
    }

    /// Displaces hypercube to any destination but makes sure hypercube stays within its
    /// initial bound
    pub fn displace_to(&mut self, destination: &Point) {
        // TODO: write tests for this method

        // ensures the destination vector is the correct dimension
        assert_eq!(
            destination.dim() as u32,
            self.dimension,
            "vector is not the correct size. \
            expected {}, got {}.",
            self.dimension,
            destination.dim()
        );

        let center_to_destination = destination - &self.center;

        // test adding destination vector to current bounds
        let new_bounds: HypercubeBounds = self.current_bounds.displace_by(&center_to_destination);

        // if new bounds are within the bounds that the hypercube was initialized with
        match new_bounds.within(&self.init_bounds) {
            BoundsOverlap::NoneOutOfBounds => {
                self.raw_displace_to(&destination);
            }
            _ => {
                // clamp new_bounds to self.init_bounds
                let clamped_bounds = new_bounds.clamp(&self.init_bounds);

                // figure out the center of the clamped bounds
                let clamped_center = clamped_bounds.compute_center();

                // ARGUMENT: since the new bounds are clamped within the init_bounds,
                // the center of the clamped bounds must be within the init_bounds

                // ARGUMENT: raw displacing to the clamped center should mean that the hypercube's
                // current bounds == clamped bounds since the size of the hypercube does not change
                // during the execution of this function

                // move the hypercube to clamped center
                self.raw_displace_to(&clamped_center);
            }
        }
    }

    /// Displaces the hypercube without any bounds checking
    fn raw_displace_to(&mut self, destination: &Point) {
        let center_to_destination = destination - &self.center;

        // add vector to bounds
        let new_bounds = self.current_bounds.displace_by(&center_to_destination);

        // current bounds should now be new_bounds
        self.current_bounds = new_bounds;

        // add destination to center
        self.center += center_to_destination.clone();

        // add destination to population
        for point in self.population.iter_mut() {
            *point += center_to_destination.clone();
        }

        // wipe out previous evaluation results
        self.values.clear();
        self.ordered_values.clear();

        // calculate new diagonal
        self.diagonal = self.current_bounds.get_upper() - self.current_bounds.get_lower();
    }

    /// Shrinks the hypercube by the given `factor`. This eliminates the previously computed
    /// hypercube values.
    pub fn shrink(&mut self, factor: f64) {
        assert!(factor > 0.0, "factor cannot be less than zero");
        assert!(factor <= 1.0, "factor cannot be more than one");

        // resize current bounds
        self.current_bounds = self
            .current_bounds
            .shrink_towards_center(&self.center, factor);

        // resize population points
        for point in self.population.iter_mut() {
            point.shrink_towards_center_in_place(&self.center, factor);
        }

        // recalculate diagonal
        self.diagonal = self.current_bounds.get_diagonal();

        // clear previous evaluation values
        self.values.clear();
        self.ordered_values.clear();
    }

    /// Re-generate points inside hypercube and erase previous evaluations
    pub fn randomize_pop(&mut self) {
        // randomize the hypercube's population
        let new_random_points = Hypercube::generate_random_points(
            self.dimension,
            self.population_size,
            self.current_bounds.get_lower().min_val().unwrap(),
            self.current_bounds.get_upper().max_val().unwrap(),
        );

        self.population = new_random_points;

        // clear previous evaluations
        self.values.clear();
        self.ordered_values.clear();
    }

    /// Generate a vector of random points with a given dimension and within given bounds
    fn generate_random_points(
        dimension: u32,
        num_points: u64,
        lower_bound: f64,
        upper_bound: f64,
    ) -> Vec<Point> {
        assert!(
            upper_bound > lower_bound,
            "upper bound not strictly larger than lower bound"
        );

        // random point Vector to store random generated points
        let mut random_points: Vec<Point> = Vec::with_capacity(num_points as usize);

        for _ in 0..num_points {
            // insert point into random_points vector
            let point = Point::random(dimension, lower_bound, upper_bound);
            random_points.push(point);
        }

        random_points
    }

    pub fn has_shrunk(&self) -> bool {
        self.current_bounds != self.init_bounds
    }

    pub fn diagonal_len(&self) -> f64 {
        self.diagonal.len()
    }

    pub fn get_population_size(&self) -> u64 {
        self.population_size
    }

    pub fn get_center(&self) -> &Point {
        &self.center
    }

    pub fn get_side_length(&self) -> f64 {
        self.current_bounds.get_length()
    }
}

impl PartialEq for Hypercube {
    fn eq(&self, other: &Self) -> bool {
        let mut bool_vec = Vec::new();

        bool_vec.push(self.dimension == other.dimension);
        bool_vec.push(self.init_bounds == other.init_bounds);
        bool_vec.push(self.current_bounds == other.current_bounds);
        bool_vec.push(self.diagonal == other.diagonal);
        bool_vec.push(self.center == other.center);
        bool_vec.push(self.population_size == other.population_size);
        bool_vec.push(self.population == other.population);

        let equal = bool_vec.into_iter().fold(true, |acc, x| acc & x);

        equal
    }
}

impl fmt::Display for Hypercube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            ">>> HYPERCUBE START:\n\
            Dimension: {}\nCurrent bounds: {:?}\
            \nCenter: {:?}\nDiagonal length: {:.2}\nPopulation size: {}\nValues: {:?}\n\
            <<< HYPERCUBE END\n",
            self.dimension,
            self.current_bounds,
            self.center,
            self.diagonal.len(),
            self.population_size,
            self.values
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objective_functions::rastrigin;

    #[test]
    fn new_hypercube_1() {
        let test_hypercube = Hypercube::new(3, 34.0, 120.0);

        let expected_bounds = HypercubeBounds::new(3, 34.0, 120.0);

        assert_eq!(test_hypercube.current_bounds, expected_bounds);
        assert_eq!(test_hypercube.init_bounds, expected_bounds);
        assert_eq!(
            test_hypercube.values,
            Vec::with_capacity(test_hypercube.dimension as usize)
        );
        assert_eq!(test_hypercube.diagonal, point![86.0; 3]);
        assert!(test_hypercube.population_size > 0);
        assert_eq!(test_hypercube.center, point![77.0; 3]);
        assert_eq!(test_hypercube.dimension, 3);
    }

    #[test]
    fn evaluate_hypercube_1() {
        let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
        test_hypercube.evaluate(rastrigin);
        assert!(!test_hypercube.values.is_empty());
    }

    #[test]
    fn shrink_1() {
        let mut test_hypercube = Hypercube::new(5, 0.0, 120.0);
        let original_hypercube = Hypercube::new(5, 0.0, 120.0);

        test_hypercube.evaluate(rastrigin);

        // values should not be empty
        assert!(!test_hypercube.values.is_empty());

        // shrink hypercube from center
        test_hypercube.shrink(0.5);

        // center should not change
        assert_eq!(test_hypercube.center, original_hypercube.center);

        // initial bounds should not change
        assert_eq!(test_hypercube.init_bounds, original_hypercube.init_bounds);

        // current bounds should change
        assert_eq!(
            test_hypercube.current_bounds,
            HypercubeBounds::new(5, 30.0, 90.0)
        );

        // diagonal will change
        assert_eq!(test_hypercube.diagonal, point![60.0; 5]);

        // population points should be different
        assert_ne!(test_hypercube.population, original_hypercube.population);

        // old evaluation values should have been deleted
        assert!(test_hypercube.values.is_empty());
    }

    #[test]
    #[ignore]
    fn leakage_1() {
        // check whether the hypercube points stay within the hypercube bounds at all times
        todo!()
    }

    #[test]
    fn test_best_value_ordering() {
        let dim = 3;
        let mut hut = Hypercube::new(dim, -5.0, 5.0);

        hut.evaluate(rastrigin);

        // list will start with the biggest value
        let mut evals: Vec<PointEval> = Vec::new();

        loop {
            let best_value = hut.pop_best_value();
            if best_value == None {
                break;
            }

            evals.push(best_value.unwrap());
        }

        let mut prev_val = PointEval::new(point![0.0; dim], NotNan::new(f64::MAX).unwrap());
        for eval in evals {
            assert!(
                eval <= prev_val,
                "list is not in decreasing order as expected"
            );
            prev_val = eval;
        }
    }
}
