/*
Defines the Bounds struct for use inside the Hypercube implementation
 */

#[derive(Clone, Copy)]
pub struct Bounds {
    pub lower: f64,
    pub upper: f64,
    pub length: f64
}

impl Bounds {
    pub fn new(lower: f64, upper: f64) -> Self {
        // ensure upper bound is larger than lower bound
        assert!(upper > lower, "Lower bound is bigger than upper bound or bound is empty.");

        Self {
            lower,
            upper,
            length: upper - lower
        }
    }
}