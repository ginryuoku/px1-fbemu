extern crate minifb;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 1280;
const HEIGHT: usize = 840;

const WINDOW_START_X: usize = 8;
const WINDOW_START_Y: usize = 8;

const WINDOW_HEIGHT: usize = 800;
const WINDOW_WIDTH: usize = 600;

const WINDOW_END_X: usize = WINDOW_START_X + WINDOW_HEIGHT;
const WINDOW_END_Y: usize = WINDOW_START_Y + WINDOW_WIDTH;

fn main() {
    println!("[DEBUG] Starting terminal...");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut x = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = x; // write something more funny here!
            x += 1;
            if x > 0xFFFFFF {
                x = 0;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }

}
