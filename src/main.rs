#![deny(missing_docs)]

//! Entry point for Grimheim.

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::window::WindowSettings;

fn create_window() -> Result<GlutinWindow, String> {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Grimheim", [512; 2])
        .srgb(false)
        .opengl(opengl)
        .exit_on_esc(true);
    settings.build()
}

fn main() {
    let mut window = create_window().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));

    while let Some(_e) = events.next(&mut window) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use piston::window::{AdvancedWindow, OpenGLWindow};

    #[test]
    fn make_window() {
        let window = create_window().unwrap();
        assert_eq!(window.get_title(), "Grimheim".to_string());
        assert!(window.get_exit_on_esc());
        assert!(window.is_current());
    }
}
