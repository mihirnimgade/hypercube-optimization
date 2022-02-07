use crate::point::Point;

pub struct HypercubeOptimizerResult {
    result: String,
    loops: u32,
    function_evals: u32,
    best_x: Point,
    best_f: f64,
}

impl HypercubeOptimizerResult {
    pub fn new() -> Self {
        todo!()
    }
}
