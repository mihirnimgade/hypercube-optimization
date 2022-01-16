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
        assert!(
            upper > lower,
            "upper bound is not strictly bigger than lower bound"
        );
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
        Self { lower, upper }
    }

    /// Checks if lhs bound is completely inside rhs bound. This means that the lhs bound is a
    /// subset of the rhs bound. This implies the bounds can also be equal.
    pub fn within(&self, rhs: &Self) -> bool {
        // check upper bound
        for (index, element) in self.upper.iter().enumerate() {
            // if self.upper element is bigger than rhs.upper element...
            if element > rhs.upper.get(index).unwrap() {
                return false;
            }
        }

        // check lower bound
        for (index, element) in self.lower.iter().enumerate() {
            if element < rhs.lower.get(index).unwrap() {
                return false;
            }
        }

        true
    }

    /// Displaces hypercube bounds by `vector`
    pub fn displace_by(&self, vector: &Point) -> Self {
        Self {
            lower: &self.lower + vector,
            upper: &self.upper + vector
        }
    }

    /// Displaces hypercube bounds by `vector` in-place
    pub fn displace_by_in_place(&mut self, vector: &Point) {
        self.lower = &self.lower + vector;
        self.upper = &self.upper + vector;
    }

    pub fn scale(&mut self, scale_factor: f64) {
        self.lower.scale(scale_factor);
        self.upper.scale(scale_factor);
    }
}

mod tests {
    use super::*;

    #[test]
    fn make_new_bounds_1() {
        let a = HypercubeBounds::new(5, 0.0, 10.0);

        let b = HypercubeBounds {
            lower: Point::fill(0.0, 5),
            upper: Point::fill(10.0, 5),
        };

        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    fn make_new_bounds_2() {
        let _a = HypercubeBounds::new(5, 10.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn make_new_bounds_3() {
        let _a = HypercubeBounds::new(5, 10.0, 10.0);
    }
}