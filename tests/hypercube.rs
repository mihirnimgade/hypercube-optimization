use hypercube_optimizer::hypercube::Hypercube;
use hypercube_optimizer::point;
use hypercube_optimizer::point::Point;

#[test]
fn eight_corners() {
    let mut test_hypercube = Hypercube::new(3, 0.0, 120.0);

    // shrink HC to half its size
    test_hypercube.shrink(0.50);

    test_hypercube
        .try_displace_to(&point![30.0, 30.0, 30.0])
        .is_ok();
    test_hypercube
        .try_displace_to(&point![30.0, 30.0, 90.0])
        .is_ok();

    test_hypercube
        .try_displace_to(&point![30.0, 90.0, 30.0])
        .is_ok();
    test_hypercube
        .try_displace_to(&point![30.0, 90.0, 90.0])
        .unwrap();

    test_hypercube
        .try_displace_to(&point![90.0, 30.0, 30.0])
        .unwrap();
    test_hypercube
        .try_displace_to(&point![90.0, 30.0, 90.0])
        .unwrap();

    test_hypercube
        .try_displace_to(&point![90.0, 90.0, 30.0])
        .unwrap();
    test_hypercube
        .try_displace_to(&point![90.0, 90.0, 90.0])
        .unwrap();
}

#[test]
#[should_panic]
fn eight_corners_panic() {
    let mut test_hypercube = Hypercube::new(3, 0.0, 120.0);

    // shrink HC to slightly more than half its size
    test_hypercube.shrink(0.51);

    test_hypercube
        .try_displace_to(&point![30.0, 30.0, 30.0])
        .unwrap();
}

#[test]
fn displace_to_1() {
    let mut test_hypercube = Hypercube::new(5, 30.0, 90.0);

    let small_vector = point![0.01; 5];
    let off_center = test_hypercube.get_center() + &small_vector;

    let original_hypercube = test_hypercube.clone();

    test_hypercube.displace_to(&off_center);

    println!("{}", original_hypercube);
    println!("{}", test_hypercube);

    assert!(test_hypercube == original_hypercube);
}

#[test]
#[ignore]
fn displace_to_2() {}

#[test]
#[ignore]
fn displace_to_3() {}

#[test]
fn shrink_and_try_displace_by_1() {
    let mut test_hypercube = Hypercube::new(5, 0.0, 120.0);
    let small_vector = point![1.0; 5];

    test_hypercube.shrink((59.0 / 60.0) as f64);
    assert!(test_hypercube.try_displace_by(&small_vector).is_ok());

    // displacing again should fail
    assert!(test_hypercube.try_displace_by(&small_vector).is_err());
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
