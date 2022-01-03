/*
Defines the Point data structure used to represent mathematical vectors that can be elementwise
added, subtracted, multiplied, and divided. Once a point is created, it has a defined and
unchangeable dimension which corresponds to the capacity of the underlying Vec<f64> attribute.
 */

use crate::bounds::Bounds;
use crate::hypercube::cmp;

use std::ops::{Add, AddAssign, Sub};

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use std::slice::Iter;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Point {
    dimension: u32,
    length: f64,
    coords: Vec<f64>,
}

/// Operation trait implementations

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
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

        Self::from_vec(add_result)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        for (index, element) in self.coords.iter_mut().enumerate() {
            *element += rhs.coords.get(index).unwrap();
        }

        // recompute mathematical length
        let length = self.coords.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();

        self.length = length;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
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

        Self::from_vec(sub_result)
    }
}

/// Struct method implementations

impl Point {
    /// Creates a Point struct from a vector. Consumes vector in the process.
    pub fn from_vec(vector: Vec<f64>) -> Self {
        assert_ne!(vector.len(), 0, "vector dimension cannot be zero");

        let coords: Vec<f64> = vector;

        // compute mathematical length
        let length = coords.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();

        Self {
            dimension: coords.len() as u32,
            length,
            coords,
        }
    }

    /// Creates a Point and initializes its coordinates with `element` and a dimension of `n`.
    pub fn fill(element: f64, n: u32) -> Self {
        let coords = vec![element; n as usize];

        // compute length
        let length = coords.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();

        Self {
            dimension: n,
            length,
            coords,
        }
    }

    /// Creates a Point with random coordinates within given bounds.
    pub fn random(dimension: u32, bounds: Bounds) -> Self {
        let mut rng = thread_rng();
        let uniform_range = Uniform::new_inclusive(bounds.lower, bounds.upper);

        let random_vec: Vec<f64> = (&mut rng)
            .sample_iter(uniform_range)
            .take(dimension.try_into().unwrap())
            .collect();

        Self::from_vec(random_vec)
    }

    pub fn dim(&self) -> u32 {
        self.dimension
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
mod point_tests {
    use crate::point::Point;

    #[test]
    fn make_new_point_by_fill() {
        assert_eq!(Point::fill(4.3, 10), Point::fill(4.3, 10));
    }

    #[test]
    fn make_new_point_from_vec() {
        assert_eq!(
            Point::from_vec(vec![5.2, 4.5, 33.2]),
            Point::from_vec(vec![5.2, 4.5, 33.2])
        );
    }

    #[test]
    fn adding_two_points_1() {
        let a = point![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = point![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let c = point![2.0, 4.0, 6.0, 8.0, 10.0, 12.0];

        assert_eq!(a + b, c);
    }

    #[test]
    fn adding_two_points_2() {
        let a = point![129.0, 1211.3, 492.2];
        let b = point![677.3, 4453.2, 223.1];

        let c = Point::from_vec(vec![129.0 + 677.3, 1211.3 + 4453.2, 492.2 + 223.1]);

        assert_eq!(a + b, c);
    }

    #[test]
    fn compute_length_1() {
        let a = point![1.0, 1.0, 1.0];

        let expected_length = (3.0_f64).sqrt();

        assert_eq!(a.length, expected_length);
    }

    #[test]
    fn compute_length_2() {
        let a = point![2.0, 5.0, 3.0, 7.0];

        let expected_length = (87.0_f64).sqrt();

        assert_eq!(a.length, expected_length);
    }

    #[test]
    fn compute_length_3() {
        let a = Point::from_vec(vec![4.9, 32.2, 3.1, 889.1]);

        let expected_length = (791569.27_f64).sqrt();

        assert_eq!(a.length, expected_length);
    }

    #[test]
    fn subtract_two_points_1() {
        let a = point![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = point![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let c = point![0.0; 6];

        assert_eq!(a - b, c);
    }

    #[test]
    fn subtract_two_points_2() {
        let a = point![1.0; 5];
        let b = point![1.0; 5];

        let c = point![0.0; 5];

        assert_eq!(a - b, c);
    }

    #[test]
    fn add_assign_1() {
        let mut a = Point::fill(3.0, 4);
        let b = point![2.3, 4.3, 1.2, 6.7];

        a += b;

        assert_eq!(a, point![5.3, 7.3, 4.2, 9.7]);
    }

    #[test]
    fn add_assign_2() {
        let mut a = Point::fill(5.6, 10);
        let b = Point::fill(4.4, 10);

        a += b;

        assert_eq!(a, point![10.0; 10]);
    }
}
