extern crate tiny_renderer;

use tiny_renderer::{ImageMeta, Point3D, RGBPixel, Triangle};

use std::f32;
use std::path::Path;
use std::vec::Vec;

fn flat_shading_render(obj_filename: &str, tga_filename: &str, width: usize, height: usize) {
    // dimensions of image
    let buffer: Vec<RGBPixel> = vec![
        RGBPixel {
            red: 0,
            green: 0,
            blue: 0
        };
        width * height
    ];
    let mut black = ImageMeta {
        buffer: buffer,
        width: width as i32,
        height: height as i32,
    };

    let head = tobj::load_obj(&Path::new(obj_filename));
    assert!(head.is_ok());
    let (models, _) = head.unwrap();
    //println!("number of models {}", models.len());

    let mesh = &models[0].mesh;

    let image_position = |pos| (width as f32) * (pos + 1.0) / 2.0;
    let zero_point: Point3D = Point3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut triangle: Triangle = Triangle {
        vertices: [zero_point, zero_point, zero_point],
    };

    let mut zbuffer: Vec<f32> = vec![f32::MIN; width * height];

    for f in 0..mesh.indices.len() / 3 {
        for j in 0..3 {
            let v = mesh.indices[3 * f + j] as usize;

            let x = image_position(mesh.positions[v * 3]);
            let y = image_position(mesh.positions[v * 3 + 1]);
            triangle.vertices[j] = Point3D { x: x, y: y, z: 0.0 };
        }
        black.draw_triangle(
            triangle,
            &mut zbuffer,
            &RGBPixel {
                red: rand::random::<u8>(),
                green: rand::random::<u8>(),
                blue: rand::random::<u8>(),
            },
        );
    }
    black.to_tga_image(tga_filename);
}

fn main() {
    flat_shading_render(
        "./data/african_head.obj",
        "african_head_flat_shading.tga",
        500,
        500,
    );
}
