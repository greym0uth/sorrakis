use wasm_bindgen::{JsCast, JsValue};
use web_sys::OffscreenCanvas;

type GL = web_sys::WebGlRenderingContext;

pub fn init_webgl_ctx(canvas: &OffscreenCanvas) -> Result<GL, JsValue> {
    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0.0,0.0,0.0,0.0);

    Ok(gl)
}
