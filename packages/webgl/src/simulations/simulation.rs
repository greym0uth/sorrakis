use web_sys::WebGlRenderingContext;

pub trait Simulation: Sized {
    fn update(&mut self);
    fn render(&self, gl: &WebGlRenderingContext);
}
