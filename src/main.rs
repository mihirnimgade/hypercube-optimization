use std::fmt;

// lifetime parameter suggests that the Hypercube struct only
// lasts as long as the init_point reference
struct Hypercube<'a> {
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

fn objective_function(inputs: &[i64]) -> i64 {
    0
}

fn main() {
    // create HypercubeOptimizer object here with certain parameters

    // HypercubeOptimizer will create mutable Hypercube object and manipulate it within a loop


    let initial_point: &[f64] = &[1.0, 2.0, 3.2, 4.32, 5.7, 6.6];
    let bounds: &[f64] = &[5f64, 4.0, 5.4, 6.2, 3.4, 3.2];

    let cube = Hypercube::new(initial_point, bounds);

    println!("{}", cube);

    // HypercubeOptimizer.run() should take an objective function and bounds

    /*


    */
}
