use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub};

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use crate::bounds::HypercubeBounds;
use std::slice::Iter;

/// Defines a point data structure used to represent mathematical vectors that can be elementwise
/// added, subtracted, multiplied, and divided. Once a point is created, it has a defined and
/// unchangeable dimension which corresponds to the length of the ordered tuple the point
/// represents.
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    dimension: u32,
    coords: Box<[f64]>,
}

/* Trait implementation for mathematical operations */

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        // points need to have the same dimension to be added
        assert_eq!(
            self.dimension, other.dimension,
            "addition failed: operands do not have same dimension"
        );
        assert_ne!(
            self.dimension, 0,
            "addition failed: point dimension cannot be zero"
        );

        let mut add_result = Vec::new();

        for (index, element) in self.coords.iter().enumerate() {
            add_result.push(element + other.get(index).unwrap());
        }

        Point::from_vec(add_result)
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {
        // points need to have the same dimension to be subtracted
        assert_eq!(
            self.dimension, other.dimension,
            "subtraction failed: operands do not have same dimension"
        );
        assert_ne!(
            self.dimension, 0,
            "addition failed: point dimension cannot be zero"
        );

        let mut sub_result = Vec::new();

        for (index, element) in self.coords.iter().enumerate() {
            sub_result.push(element - other.get(index).unwrap());
        }

        Point::from_vec(sub_result)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        for (index, element) in self.coords.iter_mut().enumerate() {
            *element += rhs.coords.get(index).unwrap();
        }
    }
}

/* Struct method implementations */

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
            "point dimensiona and bounds dimension do not match"
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
}
