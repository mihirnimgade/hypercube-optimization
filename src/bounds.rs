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
    fn new_bounds_1() {
        let a = HypercubeBounds::new(5, 0.0, 10.0);

        let b = HypercubeBounds {
            lower: Point::fill(0.0, 5),
            upper: Point::fill(10.0, 5),
        };

        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    fn new_bounds_2() {
        let _a = HypercubeBounds::new(5, 10.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn new_bounds_3() {
        let _a = HypercubeBounds::new(5, 10.0, 10.0);
    }

    #[test]
    fn within_subset() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, -10.0, 200.0);

        assert_eq!(a.within(&b), true);
    }

    #[test]
    fn within_equal() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);

        assert_eq!(a.within(&a), true);
    }

    #[test]
    fn not_within_right_overlap() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, 100.0, 200.0);

        assert_eq!(a.within(&b), false);
    }

    #[test]
    fn not_within_left_overlap() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, -10.0, 90.0);

        assert_eq!(a.within(&b), false);
    }

    #[test]
    fn not_within_superset() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, 30.0, 90.0);

        assert_eq!(a.within(&b), false);
    }

    #[test]
    fn not_within() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, -10.0, -5.0);

        assert_eq!(a.within(&b), false);
    }

    #[test]
    fn displace_by_1() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let displacement_vec = point![1.0, 22.3, 11.7];

        let calc_result = a.displace_by(&displacement_vec);

        let expected_result = HypercubeBounds::from_points(
        point![1.0, 22.3, 11.7],
        point![121.0, 142.3, 131.7]
        );

        assert_eq!(expected_result, calc_result);
    }

    #[test]
    fn displace_by_2() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let displacement_vec = point![0.0; 3];

        let calc_result = a.displace_by(&displacement_vec);

        let expected_result = HypercubeBounds::from_points(
            point![0.0; 3],
            point![120.0; 3]
        );

        assert_eq!(expected_result, calc_result);
    }

    #[test]
    fn displace_by_in_place_1() {
        let mut a = HypercubeBounds::new(3, 0.0, 120.0);
        let displacement_vec = point![1.0, 22.3, 11.7];

        a.displace_by_in_place(&displacement_vec);

        let expected_result = HypercubeBounds::from_points(
            point![1.0, 22.3, 11.7],
            point![121.0, 142.3, 131.7]
        );

        assert_eq!(expected_result, a);
    }

    #[test]
    fn displace_by_in_place_2() {
        let mut a = HypercubeBounds::new(3, 0.0, 120.0);
        let displacement_vec = point![0.0; 3];

        a.displace_by_in_place(&displacement_vec);

        let expected_result = HypercubeBounds::from_points(
            point![0.0; 3],
            point![120.0; 3]
        );

        assert_eq!(expected_result, a);
    }

    #[test]
    fn scale_1() {
        let mut a = HypercubeBounds::new(3, 50.0, 120.0);

        a.scale(1.0/10.0);

        let expected_result = HypercubeBounds::from_points(
            point![5.0; 3],
            point![12.0; 3]
        );

        assert_eq!(expected_result, a);
    }
}
