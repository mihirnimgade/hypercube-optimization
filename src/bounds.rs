/*
Defines the HypercubeBounds struct along with its functions for use inside the Hypercube
implementation.
 */

use crate::point;
use crate::point::Point;

#[derive(Clone, Debug, PartialEq)]
pub struct HypercubeBounds {
    lower: Point,
    upper: Point
}

impl HypercubeBounds {

    /// Creates a new HypercubeBounds struct
    pub fn new(dimension: u32, lower: f64, upper: f64) -> Self {
        assert!(upper > lower, "upper bound is not strictly bigger than lower bound");
        assert_ne!(dimension, 0, "dimension cannot be zero");

        Self {
            lower: point![lower; dimension],
            upper: point![upper; dimension],
        }
    }

    /// Creates a new HypercubeBounds struct from points; intended for internal testing
    fn from_points(lower: Point, upper: Point) -> Self {
        // ensure lower and upper Point dimensions are equivalent
        assert_eq!(lower.dimension, lower.dimension);

        Self {
            lower,
            upper,
            length: upper - lower
        }
    }
}