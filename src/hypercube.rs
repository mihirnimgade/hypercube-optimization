use std::fmt;

use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

pub struct Hypercube {
    pub dimension: i64,
    pub upper_bound: f64,
    pub lower_bound: f64,
    pub init_point: Vec<f64>,
    population: Vec<Vec<f64>>,
    pub values: Option<Vec<f64>>,
}

impl Hypercube {
    pub fn new(dimension: i64, init_point: Vec<f64>, upper_bound: f64, lower_bound: f64) -> Self {

        // initial asserts to ensure arguments are the correct size
        assert_eq!(dimension as usize, init_point.len(), "Dimension incompatible with initial point length");
        assert_ne!(init_point.len(), 0, "Initial point is empty!");

        // TODO: replace with function that takes dimension and bounds and returns number of hypercube points
        let num_points = dimension;

        let mut rng = thread_rng();
        let uniform_range = Uniform::new_inclusive(lower_bound, upper_bound);

        // random point Vector to store random generated points
        let mut random_points: Vec<Vec<f64>> = Vec::new();

        for _ in 0..num_points {
            // generates random point using uniform distribution
            let v: Vec<f64> = (&mut rng).sample_iter(uniform_range)
                .take(dimension.try_into().unwrap())
                .collect();

            // insert point into random_points vector
            random_points.push(v);
        }

        // return Hypercube object/struct
        Self {
            dimension,
            init_point,
            upper_bound,
            lower_bound,
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
