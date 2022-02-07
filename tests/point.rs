use hypercube_optimizer::point;
use hypercube_optimizer::point::Point;

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

#[test]
fn shrink_towards_center_in_place_1() {
    let mut a = point![120.0; 3];
    let center = point![60.0; 3];

    a.shrink_towards_center_in_place(&center, 0.5);
    let expected_result = point![90.0; 3];

    assert_eq!(expected_result, a);
}

#[test]
fn shrink_towards_center_in_place_2() {
    let mut a = point![120.0; 3];
    let center = point![60.0; 3];

    a.shrink_towards_center_in_place(&center, 0.0);

    // point should move entirely onto the center
    let expected_result = point![60.0; 3];

    assert_eq!(expected_result, a);
}

#[test]
fn shrink_towards_center_in_place_3() {
    let mut a = point![120.0; 3];
    let center = point![60.0; 3];

    a.shrink_towards_center_in_place(&center, 1.0);

    // point should be unchanged
    let expected_result = point![120.0; 3];

    assert_eq!(expected_result, a);
}

#[test]
#[should_panic]
fn shrink_towards_center_in_place_4() {
    let mut a = point![120.0; 3];
    let center = point![60.0; 3];

    a.shrink_towards_center_in_place(&center, 1.1);
}

#[test]
#[should_panic]
fn shrink_towards_center_in_place_5() {
    let mut a = point![120.0; 3];
    let center = point![60.0; 3];

    a.shrink_towards_center_in_place(&center, -0.1);
}
