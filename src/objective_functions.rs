// Stores test objective functions

pub mod objective_functions {

    use std::f64::consts::PI;
    use crate::point::Point;

    pub fn rastrigin(input_point: &Point) -> f64 {
        let dimension = input_point.dimension;
        let mut sum: f64 = 0.0;

        for val in input_point.iter() {
            sum += val.powf(2.0) - (10.0 * (2.0*PI*val).cos());
        }

        10.0 * dimension as f64 + sum
    }
}