pub extern crate imagefmt;
pub extern crate typenum;

pub mod geometry;
pub mod matrix;

pub use geometry::{Point, Point3D, Triangle};
use imagefmt::{ColFmt, ColType, Image};
use std::f32;
use std::vec::Vec;

#[derive(Copy, Clone)] // copy is byte to byte copy
pub struct RGBPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// T is the pixel type
#[allow(dead_code)]
pub struct ImageMeta {
    pub buffer: Vec<RGBPixel>,
    pub width: i32,
    pub height: i32,
}

#[allow(dead_code)]
impl ImageMeta {
    // shift float point coordinates in [0, N]^2 space
    // to discrete coordinates
    fn clamp(&self, num: f32) -> usize {
        if num as i32 >= self.height {
            (self.height - 1) as usize
        } else if num <= 0.0 {
            0
        } else {
            num.floor() as usize
        }
    }

    /**
     * its assumed that parameters (x,y) are already shifted
     * into the positive quadrant.
     */
    fn image_to_vector_index(&self, x: f32, y: f32) -> usize {
        let row: usize = self.clamp(self.height as f32 - y);
        let col: usize = self.clamp(x);

        ((self.width as usize) * row) + col
    }

    /**
     * Set the pixels given a (x,y) point in positive quadrant of Z^2
     * The origin in the lower left-hand corner of the image.
     */
    fn set_pixel(&mut self, x: f32, y: f32, color: &RGBPixel) {
        let idx = self.image_to_vector_index(x, y);
        self.buffer[idx] = *color;
    }

    pub fn draw_lines_segment(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: &RGBPixel) {
        let steep = self.is_steep(x0, y0, x1, y1);

        let mut func =
            |x0, y0, x1, y1| self.draw_lines_segment_second(x0, y0, x1, y1, steep, color);

        match (steep, x0 > x1, y0 > y1) {
            (true, _, false) => func(y0, x0, y1, x1),
            (true, _, true) => func(y1, x1, y0, x0),
            (false, false, _) => func(x0, y0, x1, y1),
            (false, true, _) => func(x1, y1, x0, y0),
        }
    }

    fn is_steep(&self, x0: f32, y0: f32, x1: f32, y1: f32) -> bool {
        let dy = y1 - y0;
        let dx = x1 - x0;

        dy.abs() > dx.abs()
    }

    fn draw_lines_segment_second(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        steep: bool,
        color: &RGBPixel,
    ) {
        let slope = if x1 == x0 { 0.0 } else { (y1 - y0) / (x1 - x0) };
        let mut x = x0;
        while x <= x1 {
            let y = slope * (x - x0) + y0;
            if steep {
                self.set_pixel(y, x, color);
            } else {
                self.set_pixel(x, y, color);
            }
            x += 1.0;
        }
    }

    fn draw_lines_segment_naive(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        let delta = 0.001; //step interval
        let mut t: f32 = 0.0;
        let pixel = RGBPixel {
            red: 255,
            green: 0,
            blue: 0,
        };

        while t < 1.0 {
            // from the parametric definition of line (x,y) = t(x1,y1) + (1-t)(x0,y0)
            let x_t = x0 + (x1 - x0) * t;
            let y_t = y0 + (y1 - y0) * t;
            self.set_pixel(x_t, y_t, &pixel);

            t += delta;
        }
    }

    /**
     *  Draw lines between two segments that share one and only one
     *  endpoint.
     */
    fn draw_lines_between_segments(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        color: &RGBPixel,
    ) {
        let right_slope = (y3 - y2) / (x3 - x2);
        let steep = self.is_steep(x0, y0, x1, y1);

        let mut func =
            |a0, b0, a1, b1| self.draw_lines_segment_second(a0, b0, a1, b1, false, color);

        let slope = if steep {
            (x1 - x0) / (y1 - y0)
        } else {
            (y1 - y0) / (x1 - x0)
        };

        if steep {
            let mut y = y0;
            while y <= y1 {
                let left_x = slope * (y - y0) + x0; // actually the x value we want.
                let xt = ((y - y2) / right_slope) + x2;

                func(left_x, y, xt, y);
                y += 1.0;
            }
        } else {
            let mut x = x0;
            while x <= x1 {
                let left_y = slope * (x - x0) + y0;
                let xt = ((left_y - y2) / right_slope) + x2;

                // println!("xt: {}, left_y: {}",xt, left_y);
                func(x, left_y, xt, left_y);
                x += 1.0;
            }
        }
    }

    pub fn draw_bbox(&mut self, triangle: Triangle, color: &RGBPixel) {
        let (sw, ne) = triangle.compute_bbox();

        self.draw_lines_segment(sw.x, sw.y, ne.x, sw.y, color);
        self.draw_lines_segment(sw.x, ne.y, ne.x, ne.y, color);

        self.draw_lines_segment(sw.x, sw.y, sw.x, ne.y, color);
        self.draw_lines_segment(ne.x, sw.y, ne.x, ne.y, color);
    }

    pub fn draw_triangle_old(&mut self, triangle: Triangle, color: &RGBPixel) {
        // draw outline of triangle
        let (sw, ne) = triangle.compute_bbox();

        let mut x = sw.x;

        while x <= ne.x {
            let mut y = sw.y;

            while y <= ne.y {
                let bc = triangle.barycentric_coords_f32(x, y);

                if (bc.x >= 0.0) && (bc.y >= 0.0) && (bc.z >= 0.0) {
                    self.set_pixel(x, y, color);
                }
                y += 1.0;
            }
            x += 1.0;
        }
    }

    pub fn draw_triangle(&mut self, triangle: Triangle, zbuffer: &mut Vec<f32>, color: &RGBPixel) {
        // draw outline of triangle
        let (sw, ne) = triangle.compute_bbox();

        let mut x = sw.x;

        while x <= ne.x {
            let mut y = sw.y;

            while y <= ne.y {
                let bc = triangle.barycentric_coords_f32(x, y);

                if (bc.x >= 0.0) && (bc.y >= 0.0) && (bc.z >= 0.0) {
                    let z = bc.x * triangle.vertices[0].z
                        + bc.y * triangle.vertices[1].z
                        + bc.z * triangle.vertices[2].z;
                    let idx = self.image_to_vector_index(x, y);

                    if zbuffer[idx] < z {
                        zbuffer[idx] = z;
                        self.set_pixel(x, y, color);
                    }
                }
                y += 1.0;
            }
            x += 1.0;
        }
    }

    pub fn draw_triangle_with_texture(
        &mut self,
        triangle: Triangle,
        texture_coords: [Point3D; 3],
        texture_img: &Image<u8>,
        intensity: f32,
        zbuffer: &mut Vec<f32>,
    ) {
        let tga_clamp = |num: f32, dim: usize| -> usize {
            if num as usize >= dim {
                (num as usize) % dim
            } else if num <= 0.0 {
                0
            } else {
                num.floor() as usize
            }
        };

        let index_tga_image = |u: f32, v: f32, w: usize, h: usize| -> usize {
            // u is x, v is y
            let row: usize = tga_clamp(h as f32 - v, h);
            let col: usize = tga_clamp(u, w);

            w * row + col
        };

        // draw outline of triangle
        let (sw, ne) = triangle.compute_bbox();

        let mut x = sw.x;

        while x <= ne.x {
            let mut y = sw.y;

            while y <= ne.y {
                let bc = triangle.barycentric_coords_f32(x, y);
                let up = (bc.x >= 0.0) && (bc.y >= 0.0) && (bc.z >= 0.0);
                let dn = (bc.x <= 0.0) && (bc.y <= 0.0) && (bc.z <= 0.0);

                if up || dn {
                    let z = bc.x * triangle.vertices[0].z
                        + bc.y * triangle.vertices[1].z
                        + bc.z * triangle.vertices[2].z;
                    let idx = self.image_to_vector_index(x, y);
                    let interpolated_texture = bc.x * texture_coords[0]
                        + texture_coords[1] * bc.y
                        + texture_coords[2] * bc.z;

                    if zbuffer[idx] < z {
                        zbuffer[idx] = z;

                        let texture_idx = 3 * index_tga_image(
                            interpolated_texture.x,
                            interpolated_texture.y,
                            texture_img.w,
                            texture_img.h,
                        );

                        let color = RGBPixel {
                            red: ((texture_img.buf[texture_idx + 0] as f32) * intensity) as u8,
                            green: ((texture_img.buf[texture_idx + 1] as f32) * intensity) as u8,
                            blue: ((texture_img.buf[texture_idx + 2] as f32) * intensity) as u8,
                        };
                        self.set_pixel(x, y, &color);
                    }
                }
                y += 1.0;
            }
            x += 1.0;
        }
    }

    pub fn draw_triangles_line_sweep(&mut self, mut triangle: Triangle, color: &RGBPixel) {
        // draw outline of triangle
        triangle.sort();
        let points = triangle.vertices;

        self.draw_lines_between_segments(
            points[0].x,
            points[0].y,
            points[1].x,
            points[1].y,
            points[0].x,
            points[0].y,
            points[2].x,
            points[2].y,
            color,
        );
        self.draw_lines_between_segments(
            points[1].x,
            points[1].y,
            points[2].x,
            points[2].y,
            points[0].x,
            points[0].y,
            points[2].x,
            points[2].y,
            color,
        );
    }

    pub fn to_tga_image(&self, filename: &str) -> () {
        let vec_length = (self.width * self.height * 3) as usize;
        let mut tga_buffer: Vec<u8> = Vec::with_capacity(vec_length);

        for pixel in self.buffer.iter() {
            tga_buffer.push(pixel.red);
            tga_buffer.push(pixel.green);
            tga_buffer.push(pixel.blue);
        }

        assert_eq!(tga_buffer.len(), vec_length);

        // write TGA image
        imagefmt::write(
            filename,
            self.width as usize,
            self.height as usize,
            ColFmt::RGB,
            &tga_buffer,
            ColType::Auto,
        )
        .unwrap();
        println!("image written to {}!", filename);
    }
}
