/*
Defines the Point data structure used to represent mathematical vectors that can be elementwise
added, subtracted, multiplied, and divided. Once a point is created, it has a defined and
unchangeable dimension which corresponds to the capacity of the underlying Vec<f64> attribute.
 */

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub};

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub dimension: u32,
    coords: Vec<f64>,
}

/// Trait implementations for mathematical operations

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

/// Struct method implementations

impl Point {
    /// Creates a Point struct from a vector. Consumes vector in the process.
    pub fn from_vec(vector: Vec<f64>) -> Self {
        assert_ne!(vector.len(), 0, "vector dimension cannot be zero");

        let coords: Vec<f64> = vector;

        Self {
            dimension: coords.len() as u32,
            coords,
        }
    }

    /// Creates a `Point` and initializes its coordinates with `element` and a dimension of `n`.
    pub fn fill(element: f64, n: u32) -> Self {
        assert_ne!(n, 0, "vector dimension cannot be zero");

        let coords = vec![element; n as usize];

        Self {
            dimension: n,
            coords,
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
}

/// Comparison function

// comparision function to find max and min of Vec<f64>
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
            coords: vec![4.3; 10],
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
            coords: vec![5.2, 4.5, 3.2],
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
    fn adding_two_point_refs_1() {
        let a = point![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = point![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let c = point![2.0, 4.0, 6.0, 8.0, 10.0, 12.0];

        assert_eq!(&a + &b, c);
    }

    #[test]
    fn adding_two_point_refs_2() {
        let a = point![129.0, 1211.3, 492.2];
        let b = point![677.3, 4453.2, 223.1];

        let c = Point::from_vec(vec![129.0 + 677.3, 1211.3 + 4453.2, 492.2 + 223.1]);

        assert_eq!(&a + &b, c);
    }

    #[test]
    fn compute_length_1() {
        let a = point![1.0, 1.0, 1.0];

        let expected_length = (3.0_f64).sqrt();

        assert_eq!(a.len(), expected_length);
    }

    #[test]
    fn compute_length_2() {
        let a = point![2.0, 5.0, 3.0, 7.0];

        let expected_length = (87.0_f64).sqrt();

        assert_eq!(a.len(), expected_length);
    }

    #[test]
    fn compute_length_3() {
        let a = Point::from_vec(vec![4.9, 32.2, 3.1, 889.1]);

        let expected_length = (791569.27_f64).sqrt();

        assert_eq!(a.len(), expected_length);
    }

    #[test]
    fn compute_length_4() {
        let a = Point::fill(4.0, 5);

        let expected_length = (80.0_f64).sqrt();

        assert_eq!(a.len(), expected_length);
    }

    #[test]
    fn subtract_two_points_1() {
        let a = point![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = point![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let c = point![0.0; 6];

        assert_eq!(&a - &b, c);
    }

    #[test]
    fn subtract_two_points_2() {
        let a = point![1.0; 5];
        let b = point![1.0; 5];

        let c = point![0.0; 5];

        assert_eq!(&a - &b, c);
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

    #[test]
    fn scale_in_place_1() {
        let mut a = point![2.0, 4.0, 6.0, 8.0];

        a.scale_in_place(0.5);

        assert_eq!(a, point![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn scale_in_place_2() {
        let mut a = point![2.0, 4.0, 6.0, 8.0];

        a.scale_in_place(-0.5);

        assert_eq!(a, point![-1.0, -2.0, -3.0, -4.0]);
    }

    #[test]
    fn scale_in_place_3() {
        let mut a = point![2.0, 4.0, 6.0, 8.0];

        a.scale_in_place(0.0);

        assert_eq!(a, point![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn scale_in_place_4() {
        let mut a = point![2.0, 4.0, 6.0, 8.0];

        a.scale_in_place(2.0);

        assert_eq!(a, point![4.0, 8.0, 12.0, 16.0]);
    }

    #[test]
    fn scale_1() {
        let a = point![2.0, 4.0, 6.0, 8.0];
        let b = a.scale(0.5);

        assert_eq!(b, point![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn scale_2() {
        let a = point![2.0, 4.0, 6.0, 8.0];
        let b = a.scale(-0.5);

        assert_eq!(b, point![-1.0, -2.0, -3.0, -4.0]);
    }

    #[test]
    fn scale_3() {
        let a = point![2.0, 4.0, 6.0, 8.0];
        let b = a.scale(0.0);

        assert_eq!(b, point![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn scale_4() {
        let a = point![2.0, 4.0, 6.0, 8.0];
        let b = a.scale(2.0);

        assert_eq!(b, point![4.0, 8.0, 12.0, 16.0]);
    }
}
