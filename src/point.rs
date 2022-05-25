use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use crate::bounds::HypercubeBounds;
use std::slice::Iter;

use rayon::prelude::*;

/// Defines a point data structure used to represent mathematical vectors that can be elementwise
/// added, subtracted, multiplied, and divided. Once a point is created, it has a defined and
/// unchangeable dimension which corresponds to the length of the ordered tuple the point
/// represents.
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    dimension: u32,
    coords: Box<[f64]>,
}

/* <----- Trait implementations for mathematical operations -----> */

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {

        // step 1: parallel zip both iterators
        // step 2: parallel map over single zipped iterator

        let point_one_iter = self.coords.into_par_iter();
        let point_two_iter = other.coords.into_par_iter();

        // ensures the point structs are the same size
        let zip_result = point_one_iter.zip_eq(point_two_iter);

        let map_result = zip_result.into_par_iter().map(
            |tup| tup.0 + tup.1
            );

        let final_result: Vec<f64> = map_result.collect();

        Point::from_vec(final_result)
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {
        let point_one_iter = self.coords.into_par_iter();
        let point_two_iter = other.coords.into_par_iter();

        // ensures the point structs are the same size
        let zip_result = point_one_iter.zip_eq(point_two_iter);

        let map_result = zip_result.into_par_iter().map(
            |tup| tup.0 - tup.1
            );

        let final_result: Vec<f64> = map_result.collect();

        Point::from_vec(final_result)
    }
}

impl<'a, 'b> Mul<&'b Point> for &'a Point {
    type Output = Point;

    fn mul(self, other: &'b Point) -> Point {
        assert_eq!(
            self.dimension, other.dimension,
            "element-wise multiplication failed: operands do not have same dimension"
        );
        assert_ne!(
            self.dimension, 0,
            "element-wise multiplication failed: point dimension cannot be zero"
        );

        let mut mul_result = Vec::new();

        for (index, element) in self.coords.iter().enumerate() {
            mul_result.push(element * other.get(index).unwrap());
        }

        Point::from_vec(mul_result)
    }
}

impl<'a, 'b> Div<&'b Point> for &'a Point {
    type Output = Point;

    fn div(self, other: &'b Point) -> Point {
        assert_eq!(
            self.dimension, other.dimension,
            "element-wise division failed: operands do not have same dimension"
        );
        assert_ne!(
            self.dimension, 0,
            "element-wise division failed: point dimension cannot be zero"
        );

        let mut div_result = Vec::new();

        for (index, element) in self.coords.iter().enumerate() {
            div_result.push(element / other.get(index).unwrap());
        }

        Point::from_vec(div_result)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        for (index, element) in self.coords.iter_mut().enumerate() {
            *element += rhs.coords.get(index).unwrap();
        }
    }
}

/* <----- Struct method implementations -----> */

impl Point {
    /// Creates a Point struct from a vector. Consumes vector in the process.
    pub fn from_vec(vector: Vec<f64>) -> Self {
        assert_ne!(vector.len(), 0, "vector dimension cannot be zero");

        let coords: Vec<f64> = vector;
        let box_coords = coords.into_boxed_slice();

        Self {
            dimension: box_coords.len() as u32,
            coords: box_coords,
        }
    }

    /// Creates a `Point` and initializes its coordinates with `element` and a dimension of `n`.
    pub fn fill(element: f64, n: u32) -> Self {
        assert_ne!(n, 0, "vector dimension cannot be zero");

        let coords = vec![element; n as usize];
        let box_coords = coords.into_boxed_slice();

        Self {
            dimension: n,
            coords: box_coords,
        }
    }

    /// Calculates the mathematical length of the `Point` from the origin
    pub fn len(&self) -> f64 {
        self.coords
            .iter()
            .fold(0.0, |acc, x| acc + x.powf(2.0))
            .sqrt()
    }

    /// Creates a `Point` with random coordinates within given bounds.
    pub fn random(dimension: u32, lower: f64, upper: f64) -> Self {
        assert_ne!(dimension, 0, "vector dimension cannot be zero");
        assert!(
            upper > lower,
            "upper bound not strictly bigger than lower bound"
        );

        let mut rng = thread_rng();
        let uniform_range = Uniform::new_inclusive(lower, upper);

        let random_vec: Vec<f64> = (&mut rng)
            .sample_iter(uniform_range)
            .take(dimension.try_into().unwrap())
            .collect();

        Self::from_vec(random_vec)
    }

    /// Shrink point towards a specified center. The scale factor must be
    /// such that 0.0 <= sf <= 1.0
    pub fn shrink_towards_center_in_place(&mut self, center: &Point, scale_factor: f64) {
        assert!(scale_factor >= 0.0, "scale factor cannot be negative");
        assert!(scale_factor <= 1.0, "scale factor cannot be more than 1");

        let point_to_center: Point = center - &self;
        let scaled_point_to_center = point_to_center.scale(1.0 - scale_factor);

        *self += scaled_point_to_center;
    }

    pub fn get(&self, index: usize) -> Option<&f64> {
        self.coords.get(index)
    }

    pub fn max_val(&self) -> Option<f64> {
        self.coords.iter().copied().max_by(cmp)
    }

    pub fn min_val(&self) -> Option<f64> {
        self.coords.iter().copied().min_by(cmp)
    }

    pub fn iter(&self) -> Iter<'_, f64> {
        self.coords.iter()
    }

    pub fn dim(&self) -> u32 {
        self.dimension
    }

    /// Scales the point by scale factor in-place
    pub fn scale_in_place(&mut self, scale_factor: f64) {
        // scale elements
        for element in self.coords.iter_mut() {
            *element *= scale_factor;
        }
    }

    /// Scales point by scale factor and returns new point
    pub fn scale(&self, scale_factor: f64) -> Self {
        // TODO: could implement this better
        let mut result = self.clone();
        result.scale_in_place(scale_factor);
        result
    }

    pub fn clamp(&self, bound: &HypercubeBounds) -> Point {
        assert_eq!(
            self.dim(),
            bound.get_upper().dim(),
            "point dimension and bounds dimension do not match"
        );

        let mut clipped_vector: Vec<f64> = Vec::new();

        for (index, element) in self.iter().enumerate() {
            let upper_element = bound.get_upper().get(index).unwrap();
            let lower_element = bound.get_lower().get(index).unwrap();

            if element < lower_element {
                clipped_vector.push(*lower_element);
            } else if element > upper_element {
                clipped_vector.push(*upper_element);
            } else {
                clipped_vector.push(*element);
            }
        }

        Point::from_vec(clipped_vector)
    }

    pub fn sum(&self) -> f64 {
        let mut result = 0.0;

        for element in self.iter() {
            result += element;
        }

        result
    }
}

/* Comparison function */

/// comparison function to find max and min of Vec<f64>
pub fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

/// Point creation macro
#[macro_export]
macro_rules! point {
    ( $( $x:expr ),*) => {
        {
            Point::from_vec(vec![$($x),*])
        }
    };

    ($elem:expr; $n:expr) => {
        {
            Point::fill($elem, $n)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_point_by_fill_1() {
        let a = Point::fill(4.3, 10);
        let b = Point {
            dimension: 10,
            coords: vec![4.3; 10].into_boxed_slice(),
        };

        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    fn new_point_by_fill_2() {
        let _a = Point::fill(4.3, 0);
    }

    #[test]
    fn new_point_from_vec_1() {
        let a = Point::from_vec(vec![5.2, 4.5, 3.2]);
        let b = Point {
            dimension: 3,
            coords: vec![5.2, 4.5, 3.2].into_boxed_slice(),
        };

        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    fn new_point_from_vec_2() {
        let _a = Point::from_vec(Vec::new());
    }

    #[test]
    fn new_point_random_1() {
        let a = Point::random(3, 0.0, 10.0);

        assert_eq!(a.dimension, 3);
    }

    #[test]
    #[should_panic]
    fn new_point_random_2() {
        let _a = Point::random(0, 0.0, 10.0);
    }

    #[test]
    #[should_panic]
    fn new_point_random_3() {
        let _a = Point::random(10, 10.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn new_point_random_4() {
        let _a = Point::random(10, 10.0, 10.0);
    }

    #[test]
    fn clamp_1() {
        let test_bounds = HypercubeBounds::new(3, 23.0, 34.0);
        let test_point = point![50.0, 44.0, 900.0];

        let calc_result = test_point.clamp(&test_bounds);
        let expected_result = point![34.0; 3];

        assert_eq!(calc_result, expected_result);
    }

    #[test]
    fn clamp_2() {
        let test_bounds = HypercubeBounds::new(3, 23.0, 34.0);
        let test_point = point![50.0, 30.0, 29.3];

        let calc_result = test_point.clamp(&test_bounds);
        let expected_result = point![34.0, 30.0, 29.3];

        assert_eq!(calc_result, expected_result);
    }

    #[test]
    fn clamp_3() {
        let test_bounds = HypercubeBounds::new(3, 23.0, 34.0);
        let test_point = point![25.0, 26.4, 27.1];

        let calc_result = test_point.clamp(&test_bounds);
        let expected_result = test_point;

        assert_eq!(calc_result, expected_result);
    }

    #[test]
    fn clamp_4() {
        let test_bounds = HypercubeBounds::new(3, 23.0, 34.0);
        let test_point = point![3.0, 5.2, 2.3];

        let calc_result = test_point.clamp(&test_bounds);
        let expected_result = point![23.0; 3];

        assert_eq!(calc_result, expected_result);
    }

    #[test]
    fn clamp_5() {
        let test_bounds = HypercubeBounds::new(3, 23.0, 34.0);
        let test_point = point![50.0, 20.3, 30.2];

        let calc_result = test_point.clamp(&test_bounds);
        let expected_result = point![34.0, 23.0, 30.2];

        assert_eq!(calc_result, expected_result);
    }
}
