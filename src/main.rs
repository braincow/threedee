extern crate sdl2; 
extern crate pdqsort;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;
use std::vec::Vec;
use std::thread;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Vec3d {
    // structure that holds X, Y and Z coordinates of a vector; point in space
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Debug)]
struct Triangle {
    // triangle is a simplest of forms and contaisn threwe vectors (the points of the triangle)
    p: [Vec3d;3],
}
impl Triangle {
    fn zero() -> Triangle {
        Triangle { p: 
            [
                Vec3d { x: 0.0, y: 0.0, z: 0.0},
                Vec3d { x: 0.0, y: 0.0, z: 0.0},
                Vec3d { x: 0.0, y: 0.0, z: 0.0},
            ]
        }
    }
}
struct Mesh {
    // mesh holds multiple triangles thus forming a object (3d model)
    tris: Vec<Triangle>,
}
impl Mesh {
    fn cube() -> Mesh {
        Mesh {
            tris: vec![
            // south
            Triangle { p: [ Vec3d { x: 0.0, y: 0.0, z: 0.0}, Vec3d { x: 0.0, y: 1.0, z: 0.0}, Vec3d { x: 1.0, y: 1.0, z: 0.0} ], },
            Triangle { p: [ Vec3d { x: 0.0, y: 0.0, z: 0.0}, Vec3d { x: 1.0, y: 1.0, z: 0.0}, Vec3d { x: 1.0, y: 0.0, z: 0.0} ], },
            // east
            Triangle { p: [ Vec3d { x: 1.0, y: 0.0, z: 0.0}, Vec3d { x: 1.0, y: 1.0, z: 0.0}, Vec3d { x: 1.0, y: 1.0, z: 1.0} ], },
            Triangle { p: [ Vec3d { x: 1.0, y: 0.0, z: 0.0}, Vec3d { x: 1.0, y: 1.0, z: 1.0}, Vec3d { x: 1.0, y: 0.0, z: 1.0} ], },
            // north
            Triangle { p: [ Vec3d { x: 1.0, y: 0.0, z: 1.0}, Vec3d { x: 1.0, y: 1.0, z: 1.0}, Vec3d { x: 0.0, y: 1.0, z: 1.0} ], },
            Triangle { p: [ Vec3d { x: 1.0, y: 0.0, z: 1.0}, Vec3d { x: 0.0, y: 1.0, z: 1.0}, Vec3d { x: 0.0, y: 0.0, z: 1.0} ], },
            // west
            Triangle { p: [ Vec3d { x: 0.0, y: 0.0, z: 1.0}, Vec3d { x: 0.0, y: 1.0, z: 1.0}, Vec3d { x: 0.0, y: 1.0, z: 0.0} ], },
            Triangle { p: [ Vec3d { x: 0.0, y: 0.0, z: 1.0}, Vec3d { x: 0.0, y: 1.0, z: 0.0}, Vec3d { x: 0.0, y: 0.0, z: 0.0} ], },
            // top
            Triangle { p: [ Vec3d { x: 0.0, y: 1.0, z: 0.0}, Vec3d { x: 0.0, y: 1.0, z: 1.0}, Vec3d { x: 1.0, y: 1.0, z: 1.0} ], },
            Triangle { p: [ Vec3d { x: 0.0, y: 1.0, z: 0.0}, Vec3d { x: 1.0, y: 1.0, z: 1.0}, Vec3d { x: 1.0, y: 1.0, z: 0.0} ], },
            // bottom
            Triangle { p: [ Vec3d { x: 1.0, y: 0.0, z: 1.0}, Vec3d { x: 0.0, y: 0.0, z: 1.0}, Vec3d { x: 0.0, y: 0.0, z: 0.0} ], },
            Triangle { p: [ Vec3d { x: 1.0, y: 0.0, z: 1.0}, Vec3d { x: 0.0, y: 0.0, z: 0.0}, Vec3d { x: 1.0, y: 0.0, z: 0.0} ], },
            ]
        }
    }

    fn load_obj(filename: &String) -> Mesh {
        // read in obj file
        let mut read_vec: Vec<Vec3d> = vec![];
        let mut read_tris: Vec<Triangle> = vec![];

        // open the file and read it in line by line
        let f = File::open(filename).unwrap();
        for line in BufReader::new(f).lines() {
            let data = line.unwrap();
            let split_data: Vec<&str> = data.split_whitespace().collect();
            //println!("{:?} {}", split_data, split_data.len());
            if split_data.len() == 0 {
                continue;
            }
            if split_data[0] == "v" {
                // store vertex (point)
                read_vec.push(Vec3d{ x: split_data[1].parse().unwrap(), y: split_data[2].parse().unwrap(), z: split_data[3].parse().unwrap() });
            } else if split_data[0] == "f" {
                // store triangle constructed of index points of points (v)
                let vec1: Vec3d = read_vec[split_data[1].parse::<usize>().unwrap() - 1].clone();
                let vec2: Vec3d = read_vec[split_data[2].parse::<usize>().unwrap() - 1].clone();
                let vec3: Vec3d = read_vec[split_data[3].parse::<usize>().unwrap() - 1].clone();
                read_tris.push(Triangle { p: [vec1, vec2, vec3]});
            }
        }
        // return the tri vector as a mesh
        Mesh { tris: read_tris }
    }
}

struct Mat4x4 {
    // calculation matrix for doing projection calculations in
    m: [[f64;4];4],
}

fn multiply_matrix_vector(i: &Vec3d, o: &mut Vec3d, m: &Mat4x4) {
    // multiple given vector with the matrix
    o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
    o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
    o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];

    // push result into carthusian space
    let w: f64 = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3];
    if w != 0.0 {
        o.x /= w;
        o.y /= w;
        o.z /= w;
    }
} 

pub fn main() {
    // init SDL2
    let sdl_context = sdl2::init().unwrap();
    // ... and figure out what windowing thing we should use
    let video_subsystem = sdl_context.video().unwrap();
 
    // create window and canvas with title and size
    let window = video_subsystem.window("threedee", 768, 768)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // build 3d objects here
    //let mesh_cube: Mesh = Mesh::cube();
    let mesh_cube: Mesh = Mesh::load_obj(&"teapot.obj".to_string());

    // information for the projection matrix
    let near: f64 = 0.1;
    let far: f64 = 1000.0;
    let fov: f64 = 90.0;
    let aspect_ratio: f64 = (canvas.window().size().0 / canvas.window().size().1).into();
    // convinience tangent calculation as a one off. converted from degrees to radians
    let fov_rad: f64 = 1.0 / (fov * 0.5 / 180.0 * 3.14159).tan();
    // projection matrix itself
    let mat_proj: Mat4x4 = Mat4x4 { m: [
        [
            aspect_ratio * fov_rad,
            0.0,
            0.0,
            0.0,
        ],
        [
            0.0,
            fov_rad,
            0.0,
            0.0,
        ],
        [
            0.0,
            0.0,
            far / (far - near),
            1.0,
        ],
        [
            0.0,
            0.0,
            (-far * near) / (far - near),
            0.0,
        ]
    ]};

    // time passing, theta (used for rotation)
    let mut theta: f64 = 0.0;

    // dummy camera vector
    let dummy_camera: Vec3d = Vec3d { x: 0.0, y: 0.0, z: 0.0 };

    // start event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            //println!("{:?}", event);
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // draw/do new stuff below

        // update rotation calculations
        // rotation matrix for Z axis on the cube
        let mat_rotz: Mat4x4 = Mat4x4 { m: [
            [
                theta.cos(),
                theta.sin(),
                0.0,
                0.0,
            ],
            [
                -theta.sin(),
                theta.cos(),
                0.0,
                0.0,
            ],
            [
                0.0,
                0.0,
                1.0,
                0.0,
            ],
            [
                0.0,
                0.0,
                0.0,
                1.0,
            ]
        ]};

        // rotation matrix for X axis on the cube
        let mat_rotx: Mat4x4 = Mat4x4 { m: [
            [
                1.0,
                0.0,
                0.0,
                0.0,
            ],
            [
                0.0,
                (theta * 0.5).cos(),
                (theta * 0.5).sin(),
                0.0,
            ],
            [
                0.0,
                -(theta * 0.5).sin(),
                (theta * 0.5).cos(),
                0.0,
            ],
            [
                0.0,
                0.0,
                0.0,
                1.0,
            ]
        ]};

        // clear the canvas with black color
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for tri in &mesh_cube.tris {
            // loop through all vertices in the triangle
            let mut points: [Point;3] = [ Point::new(0,0), Point::new(0,0), Point::new(0,0) ];
            let mut tri_normalize: Triangle = Triangle::zero();
            for i in 0..3 {
                // rotate (vertice) cube on Z axis
                let mut tri_rotatedz: Triangle = Triangle::zero();
                multiply_matrix_vector(&tri.p[i], &mut tri_rotatedz.p[i], &mat_rotz);
                // rotate also on the X axis
                let mut tri_rotatedx: Triangle = Triangle::zero();
                multiply_matrix_vector(&tri_rotatedz.p[i], &mut tri_rotatedx.p[i], &mat_rotx);

                // translate coordinates away from camera
                let mut tri_translated: Triangle = tri_rotatedx.clone();
                tri_translated.p[i].z += 6.0;

                // store pre projected version for later culling in a cross product calculation
                tri_normalize.p[i] = tri_translated.p[i].clone();

                // project 3d vertices in 2d space
                let mut tri_projected: Triangle = Triangle::zero();
                multiply_matrix_vector(&tri_translated.p[i], &mut tri_projected.p[i], &mat_proj);
                // scale into view
                tri_projected.p[i].x += 1.0;
                tri_projected.p[i].y += 1.0;
                tri_projected.p[i].x *= 0.5 * canvas.window().size().0 as f64;
                tri_projected.p[i].y *= 0.5 * canvas.window().size().1 as f64;

                // convert to SDL point struct
                points[i] = Point::new(tri_projected.p[i].x as i32, tri_projected.p[i].y as i32);
            }

            // BLACKBOX: I really have no idea how this cross product calculation actually works :(
            let mut line1: Vec3d = Vec3d { x: 0.0, y: 0.0, z: 0.0 };
            line1.x = tri_normalize.p[1].x - tri_normalize.p[0].x;
            line1.y = tri_normalize.p[1].y - tri_normalize.p[0].y;
            line1.z = tri_normalize.p[1].z - tri_normalize.p[0].z;
            let mut line2: Vec3d = Vec3d { x: 0.0, y: 0.0, z: 0.0 };
            line2.x = tri_normalize.p[2].x - tri_normalize.p[0].x;
            line2.y = tri_normalize.p[2].y - tri_normalize.p[0].y;
            line2.z = tri_normalize.p[2].z - tri_normalize.p[0].z;
            let mut normal: Vec3d = Vec3d { x: 0.0, y: 0.0, z: 0.0 };
            normal.x = line1.y * line2.z - line1.z * line2.y;
            normal.y = line1.z * line2.x - line1.x * line2.z;
            normal.z = line1.x * line2.y - line1.y * line2.x;
            // normalize the normal
            let l: f64 = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
            normal.x /= l;
            normal.y /= l;
            normal.z /= l;
            //println!("{:?}", normal);
            // phew! done.

            // only draw the rectangle if it is visible
            if normal.x * (tri_normalize.p[0].x - dummy_camera.x) +
                normal.y * (tri_normalize.p[0].y - dummy_camera.y) +
                normal.z * (tri_normalize.p[0].z - dummy_camera.z) < 0.0 {

                // illumination
                let mut light_direction: Vec3d = Vec3d { x: 0.0, y: 0.0, z: -1.0 };
                // normalize the light direction
                let l: f64 = (light_direction.x * light_direction.x + light_direction.y * light_direction.y + light_direction.z * light_direction.z).sqrt();
                light_direction.x /= l;
                light_direction.y /= l;
                light_direction.z /= l;
                let light_dp: f64 = normal.x * light_direction.x + normal.y * light_direction.y + normal.z * light_direction.z;
                // set draw color to gray'ish and fill in the triangle represented by points array
                canvas.set_draw_color(get_color(&light_dp));
                fill_triangle(&points, &mut canvas);

                // set draw color to white
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                //outline_triangle(&points, &mut canvas);
            }
        }

        // redraw the canvas from backbuffer
        canvas.present();
        // sleep for few microseconds to prevent endless loop
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // and update theta to allow rotation of cube to happen
        theta += 0.02;
    }
}

fn get_color(intensity: &f64) -> Color {
    let value: u8 = (255.0 * intensity) as u8;
    Color::RGB(value, value, value)
}

fn fill_bottom_flat_triangle(points: &[Point; 3], canvas: &mut Canvas<Window>) {
    //println!("fill_bottom_flat_triangle");
    let invslope1: f64 = (points[1].x as f64 - points[0].x as f64) / (points[1].y as f64 - points[0].y as f64);
    let invslope2: f64 = (points[2].x as f64 - points[0].x as f64) / (points[2].y as f64 - points[0].y as f64);
    //println!("btf {:?}", points);
    //println!("btf ({}-{})/({}-{})={}", points[2].x, points[1].x, points[2].y, points[1].y, invslope2);

    let mut curx1: f64 = points[0].x as f64;
    let mut curx2: f64 = points[0].x as f64;
    for scanline_y in points[0].y..points[1].y {
        let p1 = Point::new(curx1 as i32, scanline_y);
        let p2 = Point::new(curx2 as i32, scanline_y);
        //println!("bft {:?} -> {:?}", p1, p2);
        canvas.draw_line(p1, p2).unwrap();
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

fn fill_top_flat_triangle(points: &[Point; 3], canvas: &mut Canvas<Window>) {
    //println!("fill_top_flat_triangle");
    let invslope1: f64 = (points[2].x as f64 - points[0].x as f64) / (points[2].y as f64 - points[0].y as f64);
    let invslope2: f64 = (points[2].x as f64 - points[1].x as f64) / (points[2].y as f64 - points[1].y as f64);
    //println!("ftf {:?}", points);
    //println!("ftf ({}-{})/({}-{})={}", points[2].x, points[1].x, points[2].y, points[1].y, invslope2);

    let mut curx1: f64 = points[2].x as f64;
    let mut curx2: f64 = points[2].x as f64;
    for scanline_y in (points[0].y..points[2].y).rev() {
        let p1 = Point::new(curx1 as i32, scanline_y);
        let p2 = Point::new(curx2 as i32, scanline_y);
        //println!("ftf {:?} -> {:?}", p1, p2);
        canvas.draw_line(p1, p2).unwrap();
        curx1 -= invslope1;
        curx2 -= invslope2;
    }
}

fn fill_triangle(points: &[Point; 3], canvas: &mut Canvas<Window>) {
    // http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    // sort the points for filling
    //println!("- {:?}", points);
    let mut points_sorted = points.clone();
    // pdqsort sorts in decending order
    pdqsort::sort_by(&mut points_sorted, |a, b| b.y.cmp(&a.y));
    // .. we need them in ascending order
    points_sorted.reverse();
    //println!("+ {:?}", points_sorted);

    if points_sorted[1].y == points_sorted[2].y {
        // bottom-flat triangle
        fill_bottom_flat_triangle(&points_sorted, canvas);
    } else if points_sorted[0].y == points_sorted[1].y {
        // top-flat triangle
        fill_top_flat_triangle(&points_sorted, canvas);
    } else {
        // general case, we need to split the triangle in half
        let half_point: Point = Point::new(
            points_sorted[0].x + (((points_sorted[1].y - points_sorted[0].y) as f64 /
                (points_sorted[2].y - points_sorted[0].y) as f64 ) as f64 *
                (points_sorted[2].x - points_sorted[0].x) as f64) as i32,
            points_sorted[1].y);
        //println!("h {:?}", half_point);
        fill_bottom_flat_triangle(&[points_sorted[0], points_sorted[1], half_point], canvas);
        fill_top_flat_triangle(&[points_sorted[1], half_point, points_sorted[2]], canvas);
    }
}

fn outline_triangle(points: &[Point; 3], canvas: &mut Canvas<Window>) {
    // draw triangle (as lines) to backbuffer
    for i in 0..3 {
        // end of this line is the beginning of the next..
        let mut j = i + 1;
        // .. unless the next line is actually the first one
        j = if j == 3 { 0 } else { j };
        // draw the line
        //println!("{:?}, {:?}", points[i], points[j]);
        canvas.draw_line(points[i], points[j]).unwrap();
    }
}

// eof