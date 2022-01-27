use std::collections::BinaryHeap;
use std::fmt;

use crate::bounds::HypercubeBounds;
use crate::point;
use crate::point::Point;
use ordered_float::NotNan;

pub struct Hypercube {
    dimension: u32,
    init_bounds: HypercubeBounds,
    current_bounds: HypercubeBounds,
    diagonal: Point,
    center: Point,
    population_size: u64,
    population: Vec<Point>,
    values: Option<Vec<f64>>,
    ordered_values: BinaryHeap<NotNan<f64>>,
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
        let num_points = dimension;

        // random point Vector to store random generated points
        let mut random_points: Vec<Point> = Vec::with_capacity(num_points as usize);

        // calculate the hypercube's diagonal
        let hypercube_diagonal: Point =
            &point![upper_bound; dimension] - &point![lower_bound; dimension];

        for _ in 0..num_points {
            // insert point into random_points vector
            random_points.push(Point::random(dimension, lower_bound, upper_bound));
        }

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
            values: None,
            ordered_values: BinaryHeap::with_capacity(population_size as usize),
        }
    }

    /// Applies the vector function to all points in the population and stores it in the hypercube
    /// struct.
    pub fn evaluate(&mut self, point_function: fn(&Point) -> f64) {
        let mut values = Vec::with_capacity(self.population_size as usize);

        // iterate over population points and apply vector function
        for vec in &self.population {
            let result = point_function(vec);
            let nn_result = NotNan::new(result).unwrap();

            self.ordered_values.push(nn_result);
            values.push(result);
        }

        self.values = Some(values);
    }

    /// Peek at the maximum value evaluated by the hypercube
    pub fn peek_best_value(&self) -> Option<f64> {
        match self.ordered_values.peek() {
            Some(&v) => Some(v.into_inner()),
            None => None,
        }
    }

    /// Pop the maximum value evaluated by the hypercube
    pub fn pop_best_value(&mut self) -> Option<f64> {
        match self.ordered_values.pop() {
            Some(v) => Some(v.into_inner()),
            None => None,
        }
    }

    /// Displaces the hypercube by adding the `vector` argument to the hypercube's center.
    pub fn displace_by(&mut self, vector: &Point) -> Result<(), &'static str> {
        // ensures the destination vector is the correct dimension
        assert_eq!(
            vector.dimension as u32, self.dimension,
            "destination is not the correct dimension. \
            expected {}, got {}.",
            self.dimension, vector.dimension
        );

        // test adding destination vector to current bounds
        let new_bounds: HypercubeBounds = self.current_bounds.displace_by(vector);

        // if new bounds are within the bounds that the hypercube was initialized with
        if new_bounds.within(&self.init_bounds) {
            // add vector to all points in population
            for point in self.population.iter_mut() {
                *point += vector.clone();
            }

            // current bounds should now be new_bounds
            self.current_bounds = new_bounds;

            // alter center value
            self.center += vector.clone();

            // wipe out previous evaluation results
            self.values = None;
            self.ordered_values.clear();

            Ok(())
        } else {
            Err("cannot displace, displacement results in hypercube out of bounds")
        }
    }

    /// Displaces the hypercube by moving the center to the `destination` argument.
    pub fn displace_to(&mut self, destination: &Point) -> Result<(), &'static str> {
        // ensures the destination vector is the correct dimension
        assert_eq!(
            destination.dimension as u32, self.dimension,
            "destination is not the correct dimension. \
            expected {}, got {}.",
            self.dimension, destination.dimension
        );

        let center_to_destination = destination - &self.center;

        self.displace_by(&center_to_destination)
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
        self.values = None;
        self.ordered_values.clear();
    }
}

impl fmt::Display for Hypercube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dimension: {}\nCurrent bounds: {:?}\
            \nCenter: {:?}\nDiagonal length: {:.2}\nPopulation size: {}\nValues: {:?}\n",
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
    use crate::rastrigin;

    #[test]
    fn new_hypercube_1() {
        let test_hypercube = Hypercube::new(3, 34.0, 120.0);

        let expected_bounds = HypercubeBounds::new(3, 34.0, 120.0);

        assert_eq!(test_hypercube.current_bounds, expected_bounds);
        assert_eq!(test_hypercube.init_bounds, expected_bounds);
        assert_eq!(test_hypercube.values, None);
        assert_eq!(test_hypercube.diagonal, point![86.0; 3]);
        assert!(test_hypercube.population_size > 0);
        assert_eq!(test_hypercube.center, point![77.0; 3]);
        assert_eq!(test_hypercube.dimension, 3);
    }

    #[test]
    #[should_panic]
    fn new_hypercube_2() {
        let _test_hypercube = Hypercube::new(0, 34.0, 120.0);
    }

    #[test]
    #[should_panic]
    fn new_hypercube_3() {
        let _test_hypercube = Hypercube::new(5, 120.0, 34.0);
    }

    #[test]
    #[should_panic]
    fn new_hypercube_4() {
        let _test_hypercube = Hypercube::new(5, -3.0, -37.0);
    }

    #[test]
    fn evaluate_hypercube_1() {
        let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
        test_hypercube.evaluate(rastrigin);
        assert!(test_hypercube.values.is_some());
    }

    #[test]
    fn displace_1() {
        let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
        let small_vector = point![0.01; 5];

        assert!(test_hypercube.displace_by(&small_vector).is_err());
    }

    #[test]
    #[should_panic]
    fn displace_by_2() {
        let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
        test_hypercube.shrink(0.5);
        let small_vector = point![0.01; 7];

        test_hypercube.displace_by(&small_vector);
    }

    #[test]
    fn displace_by_3() {
        let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
        let small_vector = point![0.01; 5];

        test_hypercube.shrink(0.90);
        assert!(test_hypercube.displace_by(&small_vector).is_ok());
    }

    #[test]
    fn displace_to_1() {
        let mut test_hypercube = Hypercube::new(5, 0.0, 105.0);
        let small_vector = point![52.6; 5];

        assert!(test_hypercube.displace_to(&small_vector).is_err());
    }

    #[test]
    #[should_panic]
    fn displace_to_2() {
        let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
        let small_vector = point![0.01; 7];

        test_hypercube.displace_to(&small_vector);
    }

    #[test]
    fn shrink_and_displace_1() {
        let mut test_hypercube = Hypercube::new(5, 0.0, 120.0);
        let small_vector = point![1.0; 5];

        test_hypercube.shrink((59.0 / 60.0) as f64);
        assert!(test_hypercube.displace_by(&small_vector).is_ok());

        // displacing again should fail
        assert!(test_hypercube.displace_by(&small_vector).is_err());
    }

    #[test]
    fn shrink_1() {
        let mut test_hypercube = Hypercube::new(5, 0.0, 120.0);
        let original_hypercube = Hypercube::new(5, 0.0, 120.0);

        test_hypercube.evaluate(rastrigin);

        assert!(test_hypercube.values.is_some());

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
        assert!(test_hypercube.values.is_none());
    }

    #[test]
    fn displace_to_1() {
        // TODO: finish test
        let _test_hypercube = Hypercube::new(5, 0.0, 120.0);
    }

    #[test]
    fn eight_corners() {
        let mut test_hypercube = Hypercube::new(3, 0.0, 120.0);

        // shrink HC to half its size
        test_hypercube.shrink(0.50);

        test_hypercube
            .displace_to(&point![30.0, 30.0, 30.0])
            .unwrap();
        test_hypercube
            .displace_to(&point![30.0, 30.0, 90.0])
            .unwrap();

        test_hypercube
            .displace_to(&point![30.0, 90.0, 30.0])
            .unwrap();
        test_hypercube
            .displace_to(&point![30.0, 90.0, 90.0])
            .unwrap();

        test_hypercube
            .displace_to(&point![90.0, 30.0, 30.0])
            .unwrap();
        test_hypercube
            .displace_to(&point![90.0, 30.0, 90.0])
            .unwrap();

        test_hypercube
            .displace_to(&point![90.0, 90.0, 30.0])
            .unwrap();
        test_hypercube
            .displace_to(&point![90.0, 90.0, 90.0])
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn eight_corners_panic() {
        let mut test_hypercube = Hypercube::new(3, 0.0, 120.0);

        // shrink HC to slightly more than half its size
        test_hypercube.shrink(0.51);

        test_hypercube
            .displace_to(&point![30.0, 30.0, 30.0])
            .unwrap();
        test_hypercube
            .displace_to(&point![30.0, 30.0, 90.0])
            .unwrap();

        test_hypercube
            .displace_to(&point![30.0, 90.0, 30.0])
            .unwrap();
        test_hypercube
            .displace_to(&point![30.0, 90.0, 90.0])
            .unwrap();

        test_hypercube
            .displace_to(&point![90.0, 30.0, 30.0])
            .unwrap();
        test_hypercube
            .displace_to(&point![90.0, 30.0, 90.0])
            .unwrap();

        test_hypercube
            .displace_to(&point![90.0, 90.0, 30.0])
            .unwrap();
        test_hypercube
            .displace_to(&point![90.0, 90.0, 90.0])
            .unwrap();
    }
}
