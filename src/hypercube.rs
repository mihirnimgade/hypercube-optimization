use std::fmt;
// lifetime parameter suggests that the Hypercube struct only
// lasts as long as the init_point reference
pub struct Hypercube<'a> {
    init_point: &'a[f64],
    dimension: usize,
    bounds: &'a[f64]
}

impl<'a> Hypercube<'a> {
    pub fn new(init_point: &'a[f64], bounds: &'a[f64]) -> Hypercube<'a> {
        if bounds.len() == init_point.len() {
            Hypercube {
                init_point: init_point,
                dimension: init_point.len(),
                bounds: bounds,
            }
        } else {
            panic!("Initial point and bounds arguments do not share the same size!");
        }
    }
}

impl fmt::Display for Hypercube<'_> {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Initial point: {:?}\nDimension: {}\nBounds: {:?}", self.init_point, self.dimension, self.bounds)
    }
}
