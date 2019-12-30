use opengl_graphics::GlGraphics;
use graphics::*;
use piston::input::{RenderArgs, UpdateArgs};

pub struct App {
    gl: GlGraphics,
    time: f64
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;
    }

    pub fn init(gl: GlGraphics, args: clap::ArgMatches) -> Self {
       if let Some(address) = args.value_of("server") {
           // Connect
           println!("Gave server: {}", address);
       } else {
           // Local server
           println!("No server given");
       }
       App {
           gl: gl,
           time: 0.0
       }
    }
}
