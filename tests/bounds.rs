use hypercube_optimizer::bounds::HypercubeBounds;
use hypercube_optimizer::point;
use hypercube_optimizer::point::Point;

#[test]
fn shrink_towards_center_1() {
    let a = HypercubeBounds::new(3, 0.0, 120.0);
    let center = point![60.0; 3];

    let b = a.shrink_towards_center(&center, 0.5);
    let expected_result = HypercubeBounds::new(3, 30.0, 90.0);

    assert_eq!(expected_result, b);
}

#[test]
fn shrink_towards_center_3() {
    let a = HypercubeBounds::new(3, 0.0, 120.0);
    let center = point![60.0; 3];

    let b = a.shrink_towards_center(&center, 1.0);

    assert_eq!(a, b);
}

#[test]
#[should_panic]
fn shrink_towards_center_4() {
    let a = HypercubeBounds::new(3, 0.0, 120.0);
    let center = point![60.0; 3];

    let _b = a.shrink_towards_center(&center, 1.1);
}

#[test]
#[should_panic]
fn shrink_towards_center_5() {
    let a = HypercubeBounds::new(3, 0.0, 120.0);
    let center = point![60.0; 3];

    let _b = a.shrink_towards_center(&center, -0.1);
}

#[test]
#[should_panic]
fn shrink_towards_center_6() {
    let a = HypercubeBounds::new(3, 0.0, 120.0);
    let center = point![60.0; 4];

    let _b = a.shrink_towards_center(&center, 0.7);
}

#[test]
#[should_panic]
fn new_bounds_2() {
    let _a = HypercubeBounds::new(5, 10.0, 0.0);
}

#[test]
#[should_panic]
fn new_bounds_3() {
    let _a = HypercubeBounds::new(5, 10.0, 10.0);
}

#[test]
#[should_panic]
fn new_bounds_4() {
    let _a = HypercubeBounds::new(0, 0.0, 10.0);
}
