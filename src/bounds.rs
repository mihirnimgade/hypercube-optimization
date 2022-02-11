/*
Defines the HypercubeBounds struct along with its functions for use inside the Hypercube
implementation.
 */

use crate::point;
use crate::point::Point;

#[derive(Clone, Debug, PartialEq)]
pub struct HypercubeBounds {
    lower: Point,
    upper: Point,
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
        assert_eq!(lower.dim(), lower.dim());
        Self { lower, upper }
    }

    /// Checks if lhs bound is completely inside rhs bound. This means that the lhs bound is a
    /// subset of the rhs bound. This implies the bounds can also be equal.
    ///
    /// Returns a Result that is `true` if the `self` bound is completely geometrically contained
    /// within the `rhs` bound. If the `self` bound isn't completely within the `rhs` bound, it
    /// returns which bound (can only be one of the lower or upper bound point) is out of the
    /// `rhs` bound.
    ///
    /// Panics if the `self` bound is a superset of the `rhs` bound or if there is no overlap
    /// between the two bounds.
    ///
    /// # Arguments
    ///
    /// * `self` - the bound that will be compared to `rhs`
    /// * `rhs` - second comparison bound
    ///
    pub fn within(&self, rhs: &Self) -> Result<bool, Point> {
        let mut return_bound: Point;

        let mut lower_outside_range = false;
        let mut upper_outside_range = false;

        // check self upper bound against rhs upper bound
        for (index, element) in self.upper.iter().enumerate() {
            // if self upper bound is bigger than rhs.upper element...
            if element > rhs.upper.get(index).unwrap() {
                return_bound = self.upper.clone();
                upper_outside_range = true;
            }

            // if self upper bound is smaller than rhs lower bound
            if element < rhs.lower.get(index).unwrap() {
                panic!("self bound does not overlap with rhs bound")
            }
        }

        // check self lower bound against rhs upper bound
        for (index, element) in self.lower.iter().enumerate() {
            // if self.lower element is smaller than rhs.lower element...
            if element < rhs.lower.get(index).unwrap() {
                return_bound = self.lower.clone();
                lower_outside_range = true;
            }

            // if self lower bound is larger than rhs upper bound
            if element > rhs.upper.get(index).unwrap() {
                panic!("self bound does not overlap with rhs bound")
            }
        }

        // both upper and lower bounds should not ever be outside the `rhs` bounds
        if lower_outside_range && upper_outside_range {
            panic!("self is either superset");
        } else if lower_outside_range {
            return Result::Err(self.lower.clone());
        } else if upper_outside_range {
            return Result::Err(self.upper.clone());
        } else {
            return Result::Ok(true);
        }
    }

    /// Displaces hypercube bounds by `vector`
    pub fn displace_by(&self, vector: &Point) -> Self {
        Self {
            lower: &self.lower + vector,
            upper: &self.upper + vector,
        }
    }

    /// Displaces hypercube bounds by `vector` in-place
    pub fn displace_by_in_place(&mut self, vector: &Point) {
        self.lower = &self.lower + vector;
        self.upper = &self.upper + vector;
    }

    /// Scale lower and upper bounds by a scale factor
    pub fn scale_in_place(&mut self, scale_factor: f64) {
        self.lower.scale_in_place(scale_factor);
        self.upper.scale_in_place(scale_factor);
    }

    /// Scale bounds towards center of hypercube
    pub fn shrink_towards_center(&self, center: &Point, scale_factor: f64) -> Self {
        assert!(scale_factor >= 0.0, "negative scale factor is invalid");
        assert!(scale_factor <= 1.0, "scale factor above 1 is invalid");
        assert_eq!(
            self.lower.dim(),
            center.dim(),
            "center point dimension and bounds point dimension do not match. expected {}, got {}",
            self.lower.dim(),
            center.dim()
        );

        // TODO: rewrite this to use shrink_towards_center() when it is implemented for Point

        let mut new_lower = self.lower.clone();
        let mut new_upper = self.upper.clone();

        new_lower.shrink_towards_center_in_place(&center, scale_factor);
        new_upper.shrink_towards_center_in_place(&center, scale_factor);

        Self {
            lower: new_lower,
            upper: new_upper,
        }
    }

    pub fn get_diagonal(&self) -> Point {
        &self.upper - &self.lower
    }

    pub fn get_lower(&self) -> &Point {
        &self.lower
    }

    pub fn get_upper(&self) -> &Point {
        &self.upper
    }
}

#[cfg(test)]
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
    fn check_upper_lower_dim() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        assert_eq!(a.lower.dim(), a.upper.dim());
    }

    #[test]
    fn displace_by_2() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let displacement_vec = point![0.0; 3];

        let calc_result = a.displace_by(&displacement_vec);

        let expected_result = HypercubeBounds::from_points(point![0.0; 3], point![120.0; 3]);

        assert_eq!(expected_result, calc_result);
    }

    #[test]
    fn displace_by_in_place_1() {
        let mut a = HypercubeBounds::new(3, 0.0, 120.0);
        let displacement_vec = point![1.0, 22.3, 11.7];

        a.displace_by_in_place(&displacement_vec);

        let expected_result =
            HypercubeBounds::from_points(point![1.0, 22.3, 11.7], point![121.0, 142.3, 131.7]);

        assert_eq!(expected_result, a);
    }

    #[test]
    fn displace_by_in_place_2() {
        let mut a = HypercubeBounds::new(3, 0.0, 120.0);
        let displacement_vec = point![0.0; 3];

        a.displace_by_in_place(&displacement_vec);

        let expected_result = HypercubeBounds::from_points(point![0.0; 3], point![120.0; 3]);

        assert_eq!(expected_result, a);
    }

    #[test]
    fn scale_in_place_1() {
        let mut a = HypercubeBounds::new(3, 50.0, 120.0);

        a.scale_in_place(0.0);

        let expected_result = HypercubeBounds::from_points(point![0.0; 3], point![0.0; 3]);

        assert_eq!(expected_result, a);
    }

    #[test]
    fn displace_by_1() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let displacement_vec = point![1.0, 22.3, 11.7];

        let calc_result = a.displace_by(&displacement_vec);

        let expected_result =
            HypercubeBounds::from_points(point![1.0, 22.3, 11.7], point![121.0, 142.3, 131.7]);

        assert_eq!(expected_result, calc_result);
    }

    #[test]
    fn shrink_towards_center_2() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let center = point![60.0; 3];

        let b = a.shrink_towards_center(&center, 0.0);
        let expected_result = HypercubeBounds::from_points(center.clone(), center.clone());

        assert_eq!(expected_result, b);
    }

    // <----- .within() tests ----->

    #[test]
    fn within_subset() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, -10.0, 200.0);

        assert_eq!(a.within(&b), Result::Ok(true));
    }

    #[test]
    fn within_equal() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);

        assert_eq!(a.within(&a), Result::Ok(true));
    }

    #[test]
    fn not_within_right_overlap() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, 100.0, 200.0);

        assert_eq!(a.within(&b), Result::Err(a.lower));
    }

    #[test]
    fn not_within_left_overlap() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, -10.0, 90.0);

        assert_eq!(a.within(&b), Result::Err(a.upper));
    }

    #[test]
    #[should_panic]
    fn not_within_superset() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, 30.0, 90.0);

        let result = a.within(&b);
    }

    #[test]
    #[should_panic]
    fn not_within_no_overlap() {
        let a = HypercubeBounds::new(3, 0.0, 120.0);
        let b = HypercubeBounds::new(3, -10.0, -5.0);

        let result = a.within(&b);
    }
}
