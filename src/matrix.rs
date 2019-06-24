use geometry::Point3D;
use std::cmp::{Eq, PartialEq};
use std::f32;
use std::fmt;
use std::ops::Mul;
use std::string::String;
//use typenum::UInt;

/*
 * Definition of a Matrix3x3 and implementation.
 */
#[derive(Clone, Copy)] // copy is byte to byte copy
pub struct Matrix3x3 {
    pub buffer: [f32; 9],
}

#[derive(Clone, Copy)]
pub struct Matrix4x4 {
    pub buffer: [f32; 16],
}

impl fmt::Debug for Matrix3x3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("");
        for idx in 0..3 {
            s.push_str("[ ");
            for idy in 0..3 {
                s.push_str(&self.get(idx, idy).to_string());
                s.push_str(" ");
            }
            s.push_str("]\n");
        }
        write!(f, "{}", s)
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, other: &Matrix3x3) -> bool {
        self.buffer == other.buffer
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Matrix4x4) -> bool {
        self.buffer == other.buffer
    }
}

impl Eq for Matrix4x4 {}

impl Eq for Matrix3x3 {}

impl Matrix3x3 {
    pub fn new() -> Matrix3x3 {
        Matrix3x3 { buffer: [0.0; 9] }
    }

    pub fn identity() -> Matrix3x3 {
        Matrix3x3 {
            buffer: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    // column major indexing
    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.buffer[i + 3 * j]
    }

    // column major indexing
    pub fn set(&mut self, i: usize, j: usize, val: f32) -> () {
        self.buffer[i + 3 * j] = val;
    }
}

impl Mul<Point3D> for Matrix3x3 {
    type Output = Point3D;

    fn mul(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.get(0, 0) * other.x + self.get(0, 1) * other.y + self.get(0, 2) * other.z,
            y: self.get(1, 0) * other.x + self.get(1, 1) * other.y + self.get(1, 2) * other.z,
            z: self.get(2, 0) * other.x + self.get(2, 1) * other.y + self.get(2, 2) * other.z,
        }
    }
}

impl Matrix4x4 {
    pub fn zero() -> Matrix4x4 {
        Matrix4x4 { buffer: [0.0; 16] }
    }

    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            buffer: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    // column major indexing
    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.buffer[j + 4 * i]
    }

    // column major indexing
    pub fn set(&mut self, i: usize, j: usize, val: f32) -> () {
        self.buffer[j + 4 * i] = val;
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        let mut mat = Matrix4x4::zero();

        for i in 0..4 {
            for j in 0..4 {
                let mut ret = 0.0;
                for c in 0..4 {
                    ret += self.get(i, c) * other.get(c, j);
                }
                mat.set(i, j, ret);
            }
        }

        mat
    }
}

impl Mul<[f32; 4]> for Matrix4x4 {
    type Output = [f32; 4];

    fn mul(self, other: [f32; 4]) -> [f32; 4] {
        [
            self.get(0, 0) * other[0]
                + self.get(0, 1) * other[1]
                + self.get(0, 2) * other[2]
                + self.get(0, 3) * other[3],
            self.get(1, 0) * other[0]
                + self.get(1, 1) * other[1]
                + self.get(1, 2) * other[2]
                + self.get(1, 3) * other[3],
            self.get(2, 0) * other[0]
                + self.get(2, 1) * other[1]
                + self.get(2, 2) * other[2]
                + self.get(2, 3) * other[3],
            self.get(3, 0) * other[0]
                + self.get(3, 1) * other[1]
                + self.get(3, 2) * other[2]
                + self.get(3, 3) * other[3],
        ]
    }
}

impl Mul<[f32; 4]> for &Matrix4x4 {
    type Output = [f32; 4];

    fn mul(self, other: [f32; 4]) -> [f32; 4] {
        [
            self.get(0, 0) * other[0]
                + self.get(0, 1) * other[1]
                + self.get(0, 2) * other[2]
                + self.get(0, 3) * other[3],
            self.get(1, 0) * other[0]
                + self.get(1, 1) * other[1]
                + self.get(1, 2) * other[2]
                + self.get(1, 3) * other[3],
            self.get(2, 0) * other[0]
                + self.get(2, 1) * other[1]
                + self.get(2, 2) * other[2]
                + self.get(2, 3) * other[3],
            self.get(3, 0) * other[0]
                + self.get(3, 1) * other[1]
                + self.get(3, 2) * other[2]
                + self.get(3, 3) * other[3],
        ]
    }
}
