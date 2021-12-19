use std::fmt;
use rand::{thread_rng, Rng};
use rand::distributions::{Uniform};

pub struct Hypercube {
    dimension: i64,
    upper_bound: f64,
    lower_bound: f64,
    init_point: Vec<f64>,
    population: Vec<Vec<f64>>
}

impl Hypercube {
    pub fn new(dimension: i64, init_point: Vec<f64>, upper_bound: f64, lower_bound: f64) -> Self {
        assert!(dimension as usize == init_point.len(), "Dimension incompatible with initial point length");
        assert!(init_point.len() != 0, "Initial point is empty!");

        // TODO: replace with function that takes dimension and bounds and returns number of hypercube points
        let num_points = dimension;

        let mut rng = thread_rng();
        let uniform_range = Uniform::new_inclusive(lower_bound, upper_bound);

        // random point Vector to store random generated points
        let mut random_points : Vec<Vec<f64>> = Vec::new();

        for _ in 0..num_points {
            // generates random point using uniform distribution
            let v: Vec<f64> = (&mut rng).sample_iter(uniform_range).take(dimension.try_into().unwrap()).collect();

            // insert point into random_points vector
            random_points.push(v);
        }

        Self {
            dimension: dimension,
            init_point: init_point,
            upper_bound: upper_bound,
            lower_bound: lower_bound,
            population: random_points
        }
    }
}

impl fmt::Display for Hypercube {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Initial point: {:#?}\nDimension: {}\nLower bound: {:?}\nUpper bound: {:?}\nPopulation points: {:#?}\n",
            self.init_point, self.dimension, self.lower_bound, self.upper_bound, self.population)
    }
}
