use std::cmp::{Eq, PartialEq};
use std::f32;
use std::ops::{Add, Div, Mul, Sub};

/*
 * Definition of a 2D Point and implementation.
 */
#[derive(Debug, Copy, Clone)] // copy is byte to byte copy
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Point3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point3D {}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/*
 * Definition of a 3D Point and implementation.
 */

#[derive(Debug, Copy, Clone)] // copy is byte to byte copy
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(arr: [f32; 3]) -> Point3D {
        Point3D {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }

    pub fn to_array(self) -> [f32;3] {
    	[self.x,self.y,self.z]
    }

    pub fn zero() -> Point3D {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let _length = self.length();
        self.x = self.x / _length;
        self.y = self.y / _length;
        self.z = self.z / _length;
    }

    pub fn dot(self, other: Point3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// cross product of two Point3Ds
impl Mul for Point3D {
    type Output = Point3D;

    fn mul(self, other: Point3D) -> Point3D {
        // cross-product is the determinant of:
        // [     i      j       k    ]
        // [ self.x  self.y  self.z ]
        // [ other.x other.y other.z ]
        let x = self.y * other.z - other.y * self.z;
        let y = self.x * other.z - other.x * self.z;
        let z = self.x * other.y - other.x * self.y;

        // for current application doesn't need to be normalized.
        Point3D { x: x, y: -y, z: z }
    }
}

impl Div<f32> for Point3D {
    type Output = Point3D;

    fn div(self, rhs: f32) -> Point3D {
        Point3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// multiplication on the right by a Point3D
impl Mul<Point3D> for f32 {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Point3D {
        Point3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

// multiplication on the right by f32
impl Mul<f32> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f32) -> Point3D {
        Point3D {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

#[allow(dead_code)]
struct LazyDeterminant {
    func: Box<Fn(&Vec<Point3D>) -> f32>,
}

#[allow(dead_code)]
#[derive(Copy, Clone)] // copy is byte to byte copy
pub struct Triangle {
    pub vertices: [Point3D; 3],
}

#[allow(dead_code)]
impl Triangle {
    pub fn sort(&mut self) {
        self.vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap())
    }

    // return a pair of Points (Southwest, Northeast)
    pub fn compute_bbox(&self) -> (Point, Point) {
        let xs: Vec<f32> = self.vertices.iter().map(|point| point.x).collect();
        let ys: Vec<f32> = self.vertices.iter().map(|point| point.y).collect();

        let x_min: f32 = xs.iter().cloned().fold(f32::MAX, |acc, x| acc.min(x));
        let x_max: f32 = xs.iter().cloned().fold(0.0, |acc, x| acc.max(x));

        let y_min: f32 = ys.iter().cloned().fold(f32::MAX, |acc, y| acc.min(y));
        let y_max: f32 = ys.iter().cloned().fold(0.0, |acc, y| acc.max(y));

        // southwest point && northeast point
        // Must return the pixel coords ( as f23 ) the points are in; so the
        // complete triangle is contained in the bbox.
        (
            Point {
                x: x_min.floor(),
                y: y_min.floor(),
            },
            Point {
                x: x_max.ceil(),
                y: y_max.ceil(),
            },
        )
    }

    pub fn barycentric_coords_f32(&self, x: f32, y: f32) -> Point3D {
        let point = Point3D { x: x, y: y, z: 0.0 };
        self.barycentric_coords(point)
    }

    pub fn barycentric_coords(&self, point: Point3D) -> Point3D {
        let edges: Vec<Point3D> = self.vertices.iter().map(|vert| *vert - point).collect();

        // cross-product is the determinant of:
        // [     i          j          k      ]
        // [ edges[0].x edges[1].x edges[2].x ]
        // [ edges[0].y edges[1].y edges[2].y ]
        let bc = Point3D {
            x: edges[1].x * edges[2].y - edges[1].y * edges[2].x,
            y: edges[0].y * edges[2].x - edges[0].x * edges[2].y,
            z: edges[0].x * edges[1].y - edges[0].y * edges[1].x,
        };

        bc / (bc.x.abs() + bc.y.abs() + bc.z.abs())
    }

    pub fn point_in_triangle_opt(&self, x: f32, y: f32) -> bool {
        fn determinant_2d(left_col: &Point3D, right_col: &Point3D) -> f32 {
            left_col.x * right_col.y - left_col.y * right_col.x
        }

        let point = Point3D { x: x, y: y, z: 0.0 };
        let edges: Vec<Point3D> = self.vertices.iter().map(|vert| *vert - point).collect();

        let lazy_barycentric_coords = [
            LazyDeterminant {
                func: Box::new(|cols: &Vec<Point3D>| determinant_2d(&cols[1], &cols[2])),
            },
            LazyDeterminant {
                func: Box::new(|cols: &Vec<Point3D>| -determinant_2d(&cols[0], &cols[2])),
            },
            LazyDeterminant {
                func: Box::new(|cols: &Vec<Point3D>| determinant_2d(&cols[0], &cols[1])),
            },
        ];

        for idx in 0..2 {
        	if ((lazy_barycentric_coords[idx].func)(&edges) > 0.0) != ((lazy_barycentric_coords[idx+1].func)(&edges) > 0.0) {
        		return false;
        	}
        }

        true
    }

    pub fn point_in_triangle(&self, x: f32, y: f32) -> bool {
        let point = Point3D { x: x, y: y, z: 0.0 };
        let bc = self.barycentric_coords(point);

        // the entire OR chain will not evaluate if the first
        // or the second coordinate is < 0. See https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators
        if bc.x < 0.0 || bc.y < 0.0 || bc.z < 0.0 {
            false
        } else {
            true
        }
    }
}
