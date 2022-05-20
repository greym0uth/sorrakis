use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("error creating shader program"))?;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();

    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("unknown error creating program obj")))
    }
}

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error Creating Shader!"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("unable to get shader info log")))
    }
}

pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = 1.; // 1 0 0 0
    return_var[5] = 1.; // 0 1 0 0
    return_var[10] = 1.; // 0 0 1 0
    return_var[15] = 1.; // 0 0 0 1

    return_var[12] = tx;
    return_var[13] = ty;
    return_var[14] = tz;

    return_var
}

pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];
    return_var[0] = sx;
    return_var[5] = sy;
    return_var[10] = sz;
    return_var[15] = 1.;

    return_var
}

//TODO 4x4 matrix mult function
/*
pub fn mult_matrix_4(a: [f32; 16], b:[f32; 16]) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = a[0] * b[0] + a1
}
*/
