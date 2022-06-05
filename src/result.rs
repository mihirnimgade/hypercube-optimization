use crate::{point::Point, evaluation::PointEval};

/// Exit codes:
/// 0 => successful execution
/// 1 => general optimization error
/// 2 => non-convergence within defined bounds
/// 3 => optimization timeout
/// 4 => optimization bounds are too large


#[derive(Debug)]
pub struct HypercubeOptimizerResult {
    exit_code: u32,
    message: &'static str,
    loops: u32,
    fn_evals: u32,
    best_x: Option<Point>,
    best_f: Option<f64>,
}

impl HypercubeOptimizerResult {
    pub fn new(
        exit_code: u32,
        loops: u32,
        fn_evals: u32,
        best_value: Option<&PointEval>,
    ) -> Self {
        // map exit code to message
        let message = Self::map_to_message(exit_code);

        // separate best value into point and eval

        let best_f = best_value.map(|v| v.get_eval());
        let best_x = best_value.map(|v| v.get_point());

        Self {
            exit_code,
            message,
            loops,
            fn_evals,
            best_x,
            best_f,
        }
    }

    pub fn map_to_message(exit_code: u32) -> &'static str {
        match exit_code {
            0 => "optimization successful",
            1 => "general optimization error",
            2 => "non-convergence within defined bounds",
            3 => "optimization timeout",
            4 => "optimization bounds are too large",
            _ => "",
        }
    }
}
