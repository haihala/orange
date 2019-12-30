extern crate piston;

use piston::window::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use graphics::*;
use glutin_window::GlutinWindow as Window;
use crate::piston::RenderEvent;


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Hello Piston!", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |context, graphics| {
                clear([1.0; 4], graphics);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [0.0, 0.0, 100.0, 100.0],
                          context.transform,
                          graphics);
            });
        }
    }
}
