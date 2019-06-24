extern crate tiny_renderer;

use tiny_renderer::{matrix::Matrix4x4, ImageMeta, Point3D, RGBPixel, Triangle};

use std::f32;
use std::path::Path;
use std::vec::Vec;
use tiny_renderer::imagefmt::{ColFmt, Image};

fn make_projection_matrix(camera: Point3D) -> Matrix4x4 {
    Matrix4x4 {
        buffer: [
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            -1.0 / camera.z,
            1.0,
        ],
    }
}

fn make_viewport_matrix(w: f32) -> Matrix4x4 {
    Matrix4x4 {
        buffer: [
            w / 2.0,
            0.0,
            0.0,
            w / 2.0,
            0.0,
            w / 2.0,
            0.0,
            w / 2.0,
            0.0,
            0.0,
            w / 2.0,
            w / 2.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    }
}

fn render_with_texture_projection(
    obj_filename: &str,
    tga_filename: &str,
    width: usize,
    height: usize,
) {
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

    let texture_img: Image<u8> =
        imagefmt::read("./data/african_head_diffuse.tga", ColFmt::Auto).unwrap();

    let head = tobj::load_obj(&Path::new(obj_filename));
    assert!(head.is_ok());
    let (models, _) = head.unwrap();

    let mesh = &models[0].mesh;
    let project_to_point_3d = |v: [f32; 4]| Point3D {
        x: v[0] / v[3],
        y: v[1] / v[3],
        z: v[2] / v[3],
    };

    /*
    *   Projection matrix -- representation a projection onto the z = 0 plane.
    *
    *   [ c/(c-z)     0      0  0 ]     [x]
    *   [      0     c/(c-z) 0  0 ]  *  [y]
    *   [      0      0      1  0 ]     [z]
    *   [      0      0      0  1 ]     [1]
    *
    *   Pipeline = [screen coordinate transform] * [transform that generates 3D projection multiplication factor]
    *   --------
    *
    *   [ w/2  0    0   w/2 ]
    *   [  0  w/2   0   w/2 ]  *
    *   [  0   0   w/2  w/2 ]
    *   [  0   0    0   1   ]

    *   [ 1   0  0     0 ]   [x]
    *   [ 0   1  0     0 ] * [y]
    *   [ 0   0  1     0 ]   [z]
    *   [ 0   0  -1/c  1 ]   [1]
    *
    */
    let camera = Point3D::new([0.0, 0.0, 3.0]);
    let pipeline = make_viewport_matrix(width as f32) * make_projection_matrix(camera);

    let mut triangle: Triangle = Triangle {
        vertices: [Point3D::zero(), Point3D::zero(), Point3D::zero()],
    };
    let mut triangle_world_coords = vec![Point3D::zero(); 3];
    let light_dir = Point3D {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    let mut texcoords = [Point3D::zero(); 3];

    for f in 0..mesh.indices.len() / 3 {
        for j in 0..3 {
            let v = mesh.indices[3 * f + j] as usize;

            texcoords[j] = Point3D {
                x: mesh.texcoords[v * 2] * (texture_img.w as f32),
                y: mesh.texcoords[v * 2 + 1] * (texture_img.h as f32),
                z: 0.0,
            };

            // screen coords
            triangle.vertices[j] = project_to_point_3d(
                &pipeline
                    * [
                        mesh.positions[v * 3],
                        mesh.positions[v * 3 + 1],
                        mesh.positions[v * 3 + 2],
                        1.0,
                    ],
            );

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
        let intensity = xprod.dot(light_dir);
        if intensity > 0.0 {
            black.draw_triangle_with_texture(
                triangle,
                texcoords,
                &texture_img,
                intensity,
                &mut zbuffer,
            );
        }
    }
    black.to_tga_image(tga_filename);
}

fn main() {
    render_with_texture_projection(
        "./data/african_head.obj",
        "african_head_with_texture_ortho.tga",
        800,
        800,
    );
    //draw_triangle("triangles.tga", 500, 500);
    //draw_triangles_line_sweep("triangles_line_sweep.tga", 500,500)
    //barycentric_coords_test();
}
