/*
Defines the Point data structure used to represent mathematical vectors that can be elementwise
added, subtracted, multiplied, and divided. Once a point is created, it has a defined and
unchangeable dimension which corresponds to the capacity of the underlying Vec<f64> attribute.
 */

use crate::hypercube::cmp;
use std::ops::{Add, Sub};

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Point {
    dimension: u32,
    length: f64,
    coords: Vec<f64>,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // points need to have the same dimension to be added
        assert_eq!(self.dimension, other.dimension, "addition failed: operands do not have same dimension");
        assert_ne!(self.dimension, 0, "addition failed: point dimension cannot be zero");

        let mut add_result = Vec::new();

        for (index, element) in self.coords.iter().enumerate() {
            add_result.push(element + other.get(index).unwrap());
        }

        Self::from_vec(add_result)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // points need to have the same dimension to be subtracted
        assert_eq!(self.dimension, other.dimension, "subtraction failed: operands do not have same dimension");
        assert_ne!(self.dimension, 0, "addition failed: point dimension cannot be zero");

        let mut sub_result = Vec::new();

        for (index, element) in self.coords.iter().enumerate() {
            sub_result.push(element - other.get(index).unwrap());
        }

        Self::from_vec(sub_result)
    }
}

impl IntoIterator for Point {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.coords.into_iter()
    }
}

impl Point {

    /*
    Creates a Point struct from a vector. Consumes vector in the process.
     */
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

    /*
    Creates a Point and initializes its coordinates with `element` and a dimension of `n`.
     */
    pub fn fill(element: f64, n: u32) -> Self {
        let coords = vec![element; n as usize];

        // compute length
        let length = coords.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();

        Self {
            dimension: n,
            length,
            coords
        }
    }

    fn push(&mut self, value: f64) -> Result<(), String> {
        // if the coordinate is full
        if self.coords.len() >= self.coords.capacity() {
            Err(format!("cannot push value into Point struct, dimension limit of {} reached", self.dimension))
        } else {
            self.coords.push(value);

            // recompute length
            self.length = self.coords.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();

            Ok(())
        }
    }

    fn pop(&mut self) -> Option<f64> {
        let result = self.coords.pop();

        // recompute length
        self.length = self.coords.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();

        result
    }

    pub fn get(&self, index: usize) -> Option<&f64> {
        self.coords.get(index)
    }

    pub fn max(&self) -> Option<f64> {
        self.coords.iter().copied().max_by(cmp)
    }

    pub fn min(&self) -> Option<f64> {
        self.coords.iter().copied().min_by(cmp)
    }
}

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
        assert_eq!(Point::from_vec(vec![5.2, 4.5, 33.2]), Point::from_vec(vec![5.2, 4.5, 33.2]));
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
        let mut a = Point::from_vec(vec![4.9, 32.2, 3.1, 889.1]);

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
}
