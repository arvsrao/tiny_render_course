extern crate tiny_renderer;

use tiny_renderer::geometry::Point3D;
use tiny_renderer::matrix::Matrix3x3;

#[test]
fn matrix_creation() {
    let zero_mat = Matrix3x3::new();
    let identity = Matrix3x3::identity();

    //  println!("zero matrix {:?}", zero_mat);
    //  println!("identity matrix {:?}", identity);

    for idx in 0..3 {
        for idy in 0..3 {
            assert_eq!(zero_mat.get(idx, idy), 0.0);

            if idx == idy {
                assert_eq!(identity.get(idx, idy), 1.0);
            } else {
                assert_eq!(identity.get(idx, idy), 0.0);
            }
        }
    }
}

/*#[test]
fn right_matrix_multiply() {
    /*
    [ 0 -1 0   ]   [3]   [-5]
    [ 1  1 0   ] * [5] = [8]
    [ 0  0 0.5 ]   [2]   [1]
    */
let mat = Matrix {
n: 3,
m: 3,
buffer: vec![0.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.5],
};

let vec = Matrix {
n: 3,
m: 1,
buffer: vec![3.0, 5.0, 2.0],
};

let result = mat * vec;
let expected = Matrix {
n: 3,
m: 1,
buffer: vec![-5.0, 8.0, 1.0],
};

assert_eq!(result, expected);
}*/

#[test]
fn right_point3d_multiply() {
    /*
    [ 0 -1 0   ]   [3]   [-5]
    [ 1  1 0   ] * [5] = [8]
    [ 0  0 0.5 ]   [2]   [1]
    */
    let mat = Matrix3x3 {
        buffer: [0.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.5],
    };

    let point = Point3D::new([3.0, 5.0, 2.0]);

    let result = mat * point;
    let expected = Point3D::new([-5.0, 8.0, 1.0]);

    assert_eq!(result, expected);
}
