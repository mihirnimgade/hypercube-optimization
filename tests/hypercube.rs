use hypercube_optimizer::hypercube::Hypercube;
use hypercube_optimizer::point;
use hypercube_optimizer::point::Point;

#[test]
fn eight_corners() {
    let mut test_hypercube = Hypercube::new(3, 0.0, 120.0);

    // shrink HC to half its size
    test_hypercube.shrink(0.50);

    test_hypercube
        .displace_to(&point![30.0, 30.0, 30.0])
        .unwrap();
    test_hypercube
        .displace_to(&point![30.0, 30.0, 90.0])
        .unwrap();

    test_hypercube
        .displace_to(&point![30.0, 90.0, 30.0])
        .unwrap();
    test_hypercube
        .displace_to(&point![30.0, 90.0, 90.0])
        .unwrap();

    test_hypercube
        .displace_to(&point![90.0, 30.0, 30.0])
        .unwrap();
    test_hypercube
        .displace_to(&point![90.0, 30.0, 90.0])
        .unwrap();

    test_hypercube
        .displace_to(&point![90.0, 90.0, 30.0])
        .unwrap();
    test_hypercube
        .displace_to(&point![90.0, 90.0, 90.0])
        .unwrap();
}

#[test]
#[should_panic]
fn eight_corners_panic() {
    let mut test_hypercube = Hypercube::new(3, 0.0, 120.0);

    // shrink HC to slightly more than half its size
    test_hypercube.shrink(0.51);

    test_hypercube
        .displace_to(&point![30.0, 30.0, 30.0])
        .unwrap();
}

#[test]
fn displace_by_1() {
    let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
    let small_vector = point![0.01; 5];

    assert!(test_hypercube.displace_by(&small_vector).is_err());
}

#[test]
#[should_panic]
fn displace_by_2() {
    let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
    test_hypercube.shrink(0.5);
    let small_vector = point![0.01; 7];

    test_hypercube.displace_by(&small_vector);
}

#[test]
fn displace_by_3() {
    let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
    let small_vector = point![0.01; 5];

    test_hypercube.shrink(0.90);
    assert!(test_hypercube.displace_by(&small_vector).is_ok());
}

#[test]
fn displace_to_1() {
    let mut test_hypercube = Hypercube::new(5, 0.0, 105.0);
    let small_vector = point![52.6; 5];

    assert!(test_hypercube.displace_to(&small_vector).is_err());
}

#[test]
#[should_panic]
fn displace_to_2() {
    let mut test_hypercube = Hypercube::new(5, 30.4, 105.0);
    let small_vector = point![0.01; 7];

    test_hypercube.displace_to(&small_vector);
}

#[test]
fn shrink_and_displace_1() {
    let mut test_hypercube = Hypercube::new(5, 0.0, 120.0);
    let small_vector = point![1.0; 5];

    test_hypercube.shrink((59.0 / 60.0) as f64);
    assert!(test_hypercube.displace_by(&small_vector).is_ok());

    // displacing again should fail
    assert!(test_hypercube.displace_by(&small_vector).is_err());
}

#[test]
#[should_panic]
fn new_hypercube_2() {
    let _test_hypercube = Hypercube::new(0, 34.0, 120.0);
}

#[test]
#[should_panic]
fn new_hypercube_3() {
    let _test_hypercube = Hypercube::new(5, 120.0, 34.0);
}

#[test]
#[should_panic]
fn new_hypercube_4() {
    let _test_hypercube = Hypercube::new(5, -3.0, -37.0);
}
