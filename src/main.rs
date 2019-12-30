extern crate piston;

extern crate clap;

use piston::window::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use glutin_window::GlutinWindow as Window;
use piston::input::{RenderEvent, UpdateEvent};

pub mod app;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Orange", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new());

    let matches = clap::App::new("Orange").version("1.0").get_matches();

	let mut prog: app::App = app::App::init(GlGraphics::new(opengl), matches); 

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            prog.render(&args);
        }
        if let Some(args) = event.update_args() {
            prog.update(&args);
        }
    }
}
