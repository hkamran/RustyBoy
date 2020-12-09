extern crate minifb;

use self::minifb::{Window, WindowOptions};
use crate::ppu::{SCREEN_W, SCREEN_H};

pub struct Screen {
    window: Window,
}

impl Screen {

    pub fn new() -> Self {
        let mut window = Window::new(
            "RustyShit",
            SCREEN_W,
            SCREEN_H,
            WindowOptions::default(),
        )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        return Screen {
            window
        }
    }

    pub fn update(&mut self, buffer: Vec<u32>) -> () {
        window
            .update_with_buffer(&buffer, SCREEN_W, SCREEN_H)
            .unwrap();
    }

}