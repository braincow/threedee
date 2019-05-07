extern crate sdl2; 

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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
    canvas.clear();
    canvas.present();

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
        // draw new stuff here

        // redraw the canvas
        canvas.present();
        // sleep for few microseconds to prevent endless loop
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
// eof