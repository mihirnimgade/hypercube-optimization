use std::collections::BinaryHeap;
use std::fmt;

use crate::bounds::HypercubeBounds;
use crate::point;
use crate::point::Point;

pub struct Hypercube {
    dimension: u32,
    init_bounds: HypercubeBounds,
    current_bounds: HypercubeBounds,
    diagonal: Point,
    center: Point,
    population_size: u64,
    population: Vec<Point>,
    values: Option<Vec<f64>>,
}

impl Hypercube {
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
        let central_value: f64 = (upper_bound - lower_bound) / 2.0;
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
        }
    }

    /*
    Applies the vector function to all points in the population and stores it in the hypercube
    object
    */
    pub fn evaluate(&mut self, point_function: fn(&Point) -> f64) {
        let mut values = Vec::with_capacity(self.population_size as usize);

        // iterate over population points and apply vector function
        for vec in &self.population {
            values.push(point_function(vec));
        }

        self.values = Some(values);
    }

    /*
    Displaces the hypercube by moving the center to the destination argument. Returns a Result
    object to indicate whether the displacement was successful or not.
     */
    pub fn displace_by(&mut self, vector: &Point) -> Result<(), &'static str> {
        // TODO: write test functions for this

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

            // alter center value
            self.center += vector.clone();

            // wipe out previous evaluation results
            self.values = None;

            Ok(())
        } else {
            Err("cannot displace, displacement results in hypercube out of bounds")
        }
    }

    /*
    Shrinks the radius of the hypercube by multiplying it with the factor argument.
     */
    pub fn shrink(&mut self, factor: f64) {
        assert!(factor > 0.0, "factor cannot be less than zero");
        assert!(factor <= 1.0, "factor cannot be more than one");

        // resize diagonal
        self.diagonal.scale(factor);

        // TODO: figure out if diagonal should be scaled or calculated from current_bounds

        // resize current bounds
        self.current_bounds = self
            .current_bounds
            .shrink_towards_center(&self.center, factor);

        // resize population points
        for point in self.population.iter_mut() {
            point.shrink_towards_center_in_place(&self.center, factor);
        }

        // could also iterate over population points and map closure onto them

        // clear previous evaluation values
        self.values = None;
    }
}

impl fmt::Display for Hypercube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dimension: {}\nBounds: {:#?}\
            \nCenter: {:#?}\nPopulation size: {}\nValues: {:#?}\n",
            self.dimension, self.init_bounds, self.center, self.population_size, self.values
        )
    }
}
