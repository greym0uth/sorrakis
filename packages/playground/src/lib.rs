use wasm_bindgen::prelude::*;
use web_sys::{OffscreenCanvas, WebGlRenderingContext};

use crate::simulations::{Boid, FallingSand, Flock, GoL, Simulation};
// use crate::simulations::GoL;

type GL = web_sys::WebGlRenderingContext;

mod common_funcs;
mod gl_setup;
mod quadtree;
mod rendering;
mod shaders;
mod simulations;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct FolioClient {
    gl: WebGlRenderingContext,
    flock: Flock,
    fallingsim: FallingSand,
    golsim: GoL,
    n: u16,
}

#[wasm_bindgen]
impl FolioClient {
    #[wasm_bindgen(constructor)]
    pub fn new(gl: WebGlRenderingContext, n: u16) -> Self {
        console_error_panic_hook::set_once();
        let width = gl.drawing_buffer_width();
        let height = gl.drawing_buffer_height();
        let gol = GoL::new(&gl, width as u32 / 10, height as u32 / 10);
        let fs = FallingSand::new(&gl, width as u32 / 10, height as u32 / 10);
        //*****let flock = Flock::new(&gl, canvas.width() / 10, canvas.height() / 10);
        let flock = Flock::new(&gl, width as u32, height as u32);

        Self {
            gl,
            flock: flock,
            fallingsim: fs,
            golsim: gol,
            n,
        }
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        match self.n {
            0 => self.flock.update(
                self.gl.drawing_buffer_width(),
                self.gl.drawing_buffer_height(),
            ),
            1 => self.fallingsim.update(),
            2 => self.golsim.update(
                self.gl.drawing_buffer_width(),
                self.gl.drawing_buffer_height(),
            ),
            _ => println!("err"),
        }
        Ok(())
    }

    pub fn render(&self) {
        self.gl.viewport(
            0,
            0,
            self.gl.drawing_buffer_width(),
            self.gl.drawing_buffer_height(),
        );
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        match self.n {
            0 => self.flock.render(&self.gl),
            1 => self.fallingsim.render(&self.gl),
            2 => self.golsim.render(&self.gl),
            _ => println!("err"),
        }

        //self.sim.render(&self.gl);
    }
}
