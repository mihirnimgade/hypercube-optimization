use std::fmt;

// lifetime parameter suggests that the Hypercube struct only
// lasts as long as the init_point reference
pub struct Hypercube<'a> {
    init_point: &'a[f64],
    dimension: usize,
    upper_bound: &'a[f64],
    lower_bound: &'a[f64]
}

impl<'a> Hypercube<'a> {
    pub fn new(init_point: &'a[f64], upper_bound: &'a[f64], lower_bound: &'a[f64]) -> Hypercube<'a> {
        if upper_bound.len() == init_point.len() && lower_bound.len() == init_point.len() {
            Hypercube {
                init_point: init_point,
                dimension: init_point.len(),
                upper_bound: upper_bound,
                lower_bound: lower_bound
            }
        } else {
            panic!("Initial point and bounds arguments do not share the same size!");
        }
    }
}

impl fmt::Display for Hypercube<'_> {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Initial point: {:?}\n Dimension: {}\n
        Lower bound: {:?}\n Upper bound: {:?}\n", self.init_point, self.dimension, self.lower_bound, self.upper_bound)
    }
}
