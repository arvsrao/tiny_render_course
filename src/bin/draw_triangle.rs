extern crate tiny_renderer;

use std::vec::Vec;
use tiny_renderer::{ImageMeta, Point3D, RGBPixel, Triangle};

fn draw_triangle(tga_filename: &str, width: usize, height: usize) {
    let buffer: Vec<RGBPixel> = vec![
        RGBPixel {
            red: 0,
            green: 0,
            blue: 0
        };
        width * height
    ];
    let blue = RGBPixel {
        red: 0,
        green: 0,
        blue: 255,
    };
    //let green = RGBPixel { red: 0, green: 255, blue: 0 };
    let yellow = RGBPixel {
        red: 255,
        green: 255,
        blue: 0,
    };

    let mut black = ImageMeta {
        buffer: buffer,
        width: width as i32,
        height: height as i32,
    };

    let triangle = Triangle {
        vertices: [
            Point3D {
                x: 420.0,
                y: 280.0,
                z: 0.0,
            },
            Point3D {
                x: 120.0,
                y: 200.0,
                z: 0.0,
            },
            Point3D {
                x: 20.0,
                y: 20.0,
                z: 0.0,
            },
        ],
    };
    black.draw_triangle_old(triangle, &blue);
    black.draw_bbox(triangle, &yellow);
    black.to_tga_image(tga_filename);
}

fn main() {
    draw_triangle("triangles.tga", 500, 500);
}
