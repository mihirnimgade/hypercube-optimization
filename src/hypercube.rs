use std::fmt;

use crate::bounds::Bounds;
use std::cmp::Ordering;
use crate::point::Point;
use crate::point;

pub struct Hypercube {
    dimension: u32,
    init_bounds: Bounds,
    current_bounds: Bounds,
    diagonal: f64,
    center: Point,
    population_size: u64,
    population: Vec<Point>,
    values: Option<Vec<f64>>,
}

impl Hypercube {
    pub fn new(dimension: u32, init_bounds: Bounds) -> Self {
        assert_ne!(dimension, 0, "dimension cannot be zero");

        // TODO: replace with function that takes dimension and bounds and returns number of hypercube points
        let num_points = dimension;

        // random point Vector to store random generated points
        let mut random_points: Vec<Point> = Vec::with_capacity(num_points as usize);

        // calculate the hypercube's diagonal
        let hypercube_diagonal: f64 = init_bounds.length;

        for _ in 0..num_points {
            // insert point into random_points vector
            random_points.push(Point::random(dimension, init_bounds));
        }

        let population_size = random_points.len() as u64;

        // generate center vector
        let central_value: f64 = (init_bounds.lower + init_bounds.upper) / 2.0;
        let center: Point = point![central_value; dimension];

        // return Hypercube struct
        Self {
            dimension,
            init_bounds,
            current_bounds: init_bounds,
            diagonal: hypercube_diagonal,
            center,
            population_size,
            population: random_points,
            values: None,
        }
    }

    // this method should mutate the Hypercube object
    pub fn evaluate(&mut self, vector_function: fn(&Vec<f64>) -> f64) {
        let mut values: Vec<f64> = Vec::new();

        // iterate over population points
        for vec in &self.population {
            values.push(vector_function(vec));
        }

        self.values = Some(values);
    }
}

impl fmt::Display for Hypercube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Initial point: {:#?}\nDimension: {}\nLower bound: {:?}\nUpper bound: {:?}\nPopulation points: {:#?}\nValues: {:#?}\n",
               self.init_point, self.dimension, self.lower_bound, self.upper_bound, self.population,
               self.values)
    }
}
