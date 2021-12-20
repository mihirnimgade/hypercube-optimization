// Stores test objective functions

pub mod objective_functions {

    use std::f64::consts::PI;

    pub fn rastrigin(input_vector: &Vec<f64>) -> f64 {
        let dimension = input_vector.len();
        let mut sum: f64 = 0.0;

        // iterating over slice of derefenced vector pointer
        for val in &*input_vector {
            sum += val.powf(2.0) - (10.0 * (2.0*PI*val).cos());
        }

        10.0 * dimension as f64 + sum
    }
}