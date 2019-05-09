// based on @OneLoneCoder tutorials on the subject
//  https://youtu.be/ih20l3pJoeU
//  https://youtu.be/XgMWc6LumG4
extern crate sdl2; 
extern crate pdqsort;
extern crate dotenv;
extern crate sdl2_triangle;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2_triangle::triangle as sdl_triangle;

use dotenv::dotenv;

use std::env;
use std::time::Duration;
use std::vec::Vec;
use std::thread;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

#[derive(Clone, Debug, Copy)]
struct Vec3d {
    // structure that holds X, Y and Z coordinates of a vector; point in space
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Debug, Copy)]
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
    fn points(self) -> [Point;3] {
        [
            Point::new(self.p[0].x as i32, self.p[0].y as i32),
            Point::new(self.p[1].x as i32, self.p[1].y as i32),
            Point::new(self.p[2].x as i32, self.p[2].y as i32)
        ]
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

#[derive(Clone, Debug, Copy)]
struct PaintLayer {
    triangle: Triangle,
    intensity: f64
}
impl PaintLayer {
    fn paint_to(self, canvas: &mut Canvas<Window>) {
        if env::var("RASTERIZE").unwrap_or("1".to_string()) == "1" {
            canvas.set_draw_color(self.get_color());
            sdl_triangle::fill_triangle(&self.triangle.points(), canvas);
        }
        if env::var("DRAW_WIREFRAME").unwrap_or("1".to_string()) == "1" {
            // set draw color to white
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            sdl_triangle::outline_triangle(&self.triangle.points(), canvas);
        }
    }

    fn get_color(self) -> Color {
        let value: u8 = (255.0 * self.intensity) as u8;
        Color::RGB(value, value, value)
    }
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
    // init env
    dotenv().ok();

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
    let mut mesh_cube: Mesh = Mesh::cube();
    if env::var("USE_OBJ").unwrap_or("1".to_string()) == "1" {
        mesh_cube = Mesh::load_obj(&env::var("OBJ_FILE").unwrap_or("teapot.obj".to_string()));
    }

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

        // loop through all vertices in the triangle
        let mut layers: Vec<PaintLayer> = vec![];
        for tri in &mesh_cube.tris {
            let mut tri_normalize: Triangle = Triangle::zero();
            // rotate (vertice) cube on Z axis
            let mut tri_rotatedz: Triangle = Triangle::zero();
            multiply_matrix_vector(&tri.p[0], &mut tri_rotatedz.p[0], &mat_rotz);
            multiply_matrix_vector(&tri.p[1], &mut tri_rotatedz.p[1], &mat_rotz);
            multiply_matrix_vector(&tri.p[2], &mut tri_rotatedz.p[2], &mat_rotz);
            // rotate also on the X axis
            let mut tri_rotatedx: Triangle = Triangle::zero();
            multiply_matrix_vector(&tri_rotatedz.p[0], &mut tri_rotatedx.p[0], &mat_rotx);
            multiply_matrix_vector(&tri_rotatedz.p[1], &mut tri_rotatedx.p[1], &mat_rotx);
            multiply_matrix_vector(&tri_rotatedz.p[2], &mut tri_rotatedx.p[2], &mat_rotx);

            // translate coordinates away from camera
            let mut tri_translated: Triangle = tri_rotatedx.clone();
            let distance: f64 = env::var("DISTANCE").unwrap_or("6.0".to_string()).parse::<f64>().unwrap();
            tri_translated.p[0].z += distance;
            tri_translated.p[1].z += distance;
            tri_translated.p[2].z += distance;

            // store pre projected version for later culling in a cross product calculation
            tri_normalize.p[0] = tri_translated.p[0].clone();
            tri_normalize.p[1] = tri_translated.p[1].clone();
            tri_normalize.p[2] = tri_translated.p[2].clone();

            // project 3d vertices in 2d space
            let mut tri_projected: Triangle = Triangle::zero();
            multiply_matrix_vector(&tri_translated.p[0], &mut tri_projected.p[0], &mat_proj);
            multiply_matrix_vector(&tri_translated.p[1], &mut tri_projected.p[1], &mat_proj);
            multiply_matrix_vector(&tri_translated.p[2], &mut tri_projected.p[2], &mat_proj);

            // scale into view
            tri_projected.p[0].x += 1.0;
            tri_projected.p[0].y += 1.0;
            tri_projected.p[0].x *= 0.5 * canvas.window().size().0 as f64;
            tri_projected.p[0].y *= 0.5 * canvas.window().size().1 as f64;
            tri_projected.p[1].x += 1.0;
            tri_projected.p[1].y += 1.0;
            tri_projected.p[1].x *= 0.5 * canvas.window().size().0 as f64;
            tri_projected.p[1].y *= 0.5 * canvas.window().size().1 as f64;
            tri_projected.p[2].x += 1.0;
            tri_projected.p[2].y += 1.0;
            tri_projected.p[2].x *= 0.5 * canvas.window().size().0 as f64;
            tri_projected.p[2].y *= 0.5 * canvas.window().size().1 as f64;

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

                // add this triangle to layer painting queue
                layers.push(PaintLayer{triangle: tri_projected, intensity: light_dp });
            }
        }

        // sort layers in correct order (draw furthes most triangle first)
        pdqsort::sort_by(&mut layers, |t1, t2| {
			let z1: f64 = (t1.triangle.p[0].z + t1.triangle.p[1].z + t1.triangle.p[2].z) / 3.0;
			let z2: f64 = (t2.triangle.p[0].z + t2.triangle.p[1].z + t2.triangle.p[2].z) / 3.0;
            if z1 < z2 {
                // z2 is greater
                Ordering::Greater
            } else if z1 > z2 {
                // z2 is less
                Ordering::Less
            } else {
                // z2 is equal
                Ordering::Equal
            }
        });
        // actually draw triangles to screen to achieve proper perspective in drawing
        for layer in layers {
            layer.paint_to(&mut canvas);
        }

        // redraw the canvas from backbuffer
        canvas.present();
        // sleep for few microseconds to prevent endless loop
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // and update theta to allow rotation of cube to happen
        theta += env::var("THETA").unwrap_or("0.002".to_string()).parse::<f64>().unwrap();
    }
}

// eof