use crate::Point;
use ordered_float::NotNan;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct PointEval {
    argument: Point,
    image: NotNan<f64>,
}

impl PointEval {
    pub fn new(argument: Point, image: NotNan<f64>) -> Self {
        Self { argument, image }
    }

    pub fn new_with_eval(argument: Point, function: fn(&Point) -> f64) -> Self {
        let image = function(&argument);
        let nn_image = NotNan::new(image);

        match nn_image {
            Ok(nn) => Self {
                argument,
                image: nn,
            },
            Err(_) => panic!("function evaluated at {:?} returned {}", argument, image),
        }
    }

    pub fn eval(&mut self, func: fn(&Point) -> f64) {
        // evaluate the function at point and insert image into struct
        let image = func(&self.argument);
        let nn_image = NotNan::new(image);

        match nn_image {
            Ok(nn) => self.image = nn,
            Err(_) => panic!(
                "function evaluated at {:?} returned {}",
                self.argument, image
            ),
        }
    }

    pub fn get_eval(&self) -> f64 {
        self.image.into_inner()
    }
}

impl PartialEq for PointEval {
    fn eq(&self, other: &Self) -> bool {
        self.image == other.image
    }
}

impl Eq for PointEval {}

impl PartialOrd for PointEval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.image.cmp(&other.image))
    }
}

impl Ord for PointEval {
    fn cmp(&self, other: &Self) -> Ordering {
        // compare only by image
        self.image.cmp(&other.image)
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluation::PointEval;
    use crate::objective_functions::objective_functions::{nan_function, summation};
    use crate::{point, rastrigin, Point};
    use ordered_float::NotNan;

    #[test]
    fn new_1() {
        let test_argument = point![1.0; 3];
        let test_image = unsafe { NotNan::new_unchecked(rastrigin(&test_argument)) };
        let test_eval = PointEval::new(test_argument.clone(), test_image);

        let expected_eval = PointEval {
            argument: test_argument,
            image: test_image,
        };

        assert_eq!(test_eval, expected_eval)
    }

    #[test]
    fn new_with_eval_1() {
        let test_point = point![0.0; 3];
        let test_eval = PointEval::new_with_eval(test_point.clone(), rastrigin);

        let expected_eval = PointEval {
            argument: test_point.clone(),
            image: NotNan::new(0.0).unwrap(),
        };
    }

    #[test]
    #[should_panic]
    fn new_with_eval_2() {
        let test_point = point![0.0; 3];
        let test_eval = PointEval::new_with_eval(test_point, nan_function);
    }

    #[test]
    fn comparison_1() {
        let test_point_a = point![0.0; 3];
        let test_point_b = point![1.0; 3];

        let test_eval_a = PointEval::new_with_eval(test_point_a, summation);
        let test_eval_b = PointEval::new_with_eval(test_point_b, summation);

        assert_eq!(test_eval_a < test_eval_b, true);
        assert_eq!(test_eval_a <= test_eval_a, true);
    }

    #[test]
    fn comparison_2() {
        let test_point_a = point![0.0; 3];
        let test_point_b = point![1.0; 3];

        let test_eval_a = PointEval::new_with_eval(test_point_a, summation);
        let test_eval_b = PointEval::new_with_eval(test_point_b, summation);

        assert_eq!(test_eval_a > test_eval_b, false);
    }

    #[test]
    fn get_eval_1() {
        let test_point = point![1.0; 3];
        let test_eval = PointEval::new_with_eval(test_point, summation);

        assert_eq!(test_eval.get_eval(), 3.0_f64);
    }

    #[test]
    fn max_1() {
        let test_point_a = point![2.0; 3];
        let test_point_b = point![4.0; 3];

        let test_eval_a = PointEval::new_with_eval(test_point_a, summation);
        let test_eval_b = PointEval::new_with_eval(test_point_b, summation);

        assert_eq!(test_eval_b.clone().max(test_eval_a), test_eval_b);
    }

    #[test]
    fn eval_1() {
        let test_point = point![1.0; 3];
        let mut test_eval = PointEval::new_with_eval(test_point, rastrigin);

        test_eval.eval(summation);

        assert_eq!(test_eval.get_eval(), 3.0_f64);
    }

    #[test]
    #[should_panic]
    fn eval_2() {
        let test_point = point![1.0; 3];
        let mut test_eval = PointEval::new_with_eval(test_point, rastrigin);

        test_eval.eval(nan_function);
    }
}
