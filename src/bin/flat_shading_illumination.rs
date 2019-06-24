extern crate tiny_renderer;

use tiny_renderer::{ImageMeta, Point3D, RGBPixel, Triangle};

use std::f32;
use std::path::Path;
use std::vec::Vec;
use tiny_renderer::imagefmt::{ColFmt, Image};

fn flat_shading_illumination(obj_filename: &str, tga_filename: &str, width: usize, height: usize) {
    // dimensions of image
    let buffer: Vec<RGBPixel> = vec![
        RGBPixel {
            red: 0,
            green: 0,
            blue: 0
        };
        width * height
    ];
    let mut zbuffer: Vec<f32> = vec![f32::MIN; width * height];
    let mut black = ImageMeta {
        buffer: buffer,
        width: width as i32,
        height: height as i32,
    };

    let head = tobj::load_obj(&Path::new(obj_filename));
    assert!(head.is_ok());
    let (models, _) = head.unwrap();

    let mesh = &models[0].mesh;

    let image_position = |pos| (width as f32) * (pos + 1.0) / 2.0;
    let zero_point_3d = Point3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut triangle: Triangle = Triangle {
        vertices: [zero_point_3d, zero_point_3d, zero_point_3d],
    };
    let mut triangle_world_coords = vec![zero_point_3d; 3];
    let light_dir = Point3D {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    for f in 0..mesh.indices.len() / 3 {
        for j in 0..3 {
            let v = mesh.indices[3 * f + j] as usize;

            // screen coords
            triangle.vertices[j] = Point3D {
                x: image_position(mesh.positions[v * 3]),
                y: image_position(mesh.positions[v * 3 + 1]),
                z: image_position(mesh.positions[v * 3 + 2]),
            };

            triangle_world_coords[j] = Point3D {
                x: mesh.positions[v * 3],
                y: mesh.positions[v * 3 + 1],
                z: mesh.positions[v * 3 + 2],
            };
        }

        let mut xprod = (triangle_world_coords[2] - triangle_world_coords[0])
            * (triangle_world_coords[1] - triangle_world_coords[0]);
        xprod.normalize();

        // light emanates from (0,0,0) an strikes the triangle in the bary center.
        //  let light_dir_length = light_dir.length(); * (1.0 / light_dir_length)
        //light_dir.normalize();
        let intensity = (xprod.dot(light_dir) * 255.0) as u8;
        if intensity > 0 {
            black.draw_triangle(
                triangle,
                &mut zbuffer,
                &RGBPixel {
                    red: intensity,
                    green: intensity,
                    blue: intensity,
                },
            );
        }
    }
    black.to_tga_image(tga_filename);
}

fn main() {
    flat_shading_illumination(
        "./data/african_head.obj",
        "african_head_lips_.tga",
        500,
        500,
    );
}
