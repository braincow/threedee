extern crate sdl2; 

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use std::time::Duration;
use std::vec::Vec;

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

struct Mat4x4 {
    // calculation matrix for doing projection calculations in
    m: [[f64;4];4],
}

fn multiply_matrix_vector(i: &Vec3d, o: &mut Vec3d, m: &Mat4x4) {
    // multiple given vector with the matrix
    o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
    o.y = i.y * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
    o.x = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];

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
    let window = video_subsystem.window("threedee", 800, 600)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // build 3d objects here
    let mut mesh_cube: Mesh = Mesh { tris: vec![] };
    mesh_cube.tris = vec![
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
    ];

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

    // start event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            println!("{:?}", event);
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // draw/do new stuff here
        // clear the canvas with black color
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for tri in &mesh_cube.tris {
            // loop through all vertices in the triangle
            let mut points: [Point;3] = [ Point::new(0,0), Point::new(0,0), Point::new(0,0) ];
            for i in 0..3 {
                // translate coordinates away from camera
                let mut tri_translated: Triangle = tri.clone();
                tri_translated.p[i].z += 1.0;
                //println!("{:?}", tri_translated.p[i]);

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
                //println!("{:?}", points[i]);
            }
            // draw triangle (as lines) to backbuffer
            for i in 0..3 {
                // end of this line is the beginning of the next..
                let mut j = i + 1;
                // .. unless the next line is actually the first one
                j = if j == 3 { 0 } else { j };
                // set draw color to white
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                // draw the line
                //println!("{:?}, {:?}", points[i], points[j]);
                canvas.draw_line(points[i], points[j]).unwrap();
            }
        }

        // redraw the canvas from backbuffer
        canvas.present();
        // sleep for few microseconds to prevent endless loop
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
// eof