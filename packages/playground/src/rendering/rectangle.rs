use crate::common_funcs as cf;
use crate::rendering::Instance;
use js_sys::{Float32Array, Uint16Array, WebAssembly};
use wasm_bindgen::JsCast;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlUniformLocation};

const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];
const VERTICES: [f32; 12] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];

pub struct Rectangle {
    indices: WebGlBuffer,
    program: WebGlProgram,
    u_color: WebGlUniformLocation,
    u_scale: WebGlUniformLocation,
    u_rotation: WebGlUniformLocation,
    u_translation: WebGlUniformLocation,
    vertices: WebGlBuffer,
}

impl Rectangle {
    pub fn new(gl: &GL) -> Self {
        let program = cf::link_program(
            &gl,
            crate::shaders::simple::VERT,
            crate::shaders::simple::FRAG,
        )
        .unwrap();

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_location = VERTICES.as_ptr() as u32 / 4;
        let vertex_array = Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + VERTICES.len() as u32);
        let vertices = gl
            .create_buffer()
            .ok_or("failed to create vertex buffer")
            .unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertices));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertex_array, GL::STATIC_DRAW);

        let indices_location = INDICES.as_ptr() as u32 / 2;
        let index_array = Uint16Array::new(&memory_buffer)
            .subarray(indices_location, indices_location + INDICES.len() as u32);
        let indices = gl
            .create_buffer()
            .ok_or("failed to create index buffer")
            .unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &index_array,
            GL::STATIC_DRAW,
        );

        let u_color = gl.get_uniform_location(&program, "u_Color").unwrap();
        let u_scale = gl.get_uniform_location(&program, "u_Scale").unwrap();
        let u_translation = gl.get_uniform_location(&program, "u_Translation").unwrap();
        let u_rotation = gl.get_uniform_location(&program, "u_Rotation").unwrap();

        Self {
            indices,
            program,
            u_color,
            u_scale,
            u_rotation,
            u_translation,
            vertices,
        }
    }

    pub fn render(&self, gl: &GL, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        gl.uniform4f(Some(&self.u_color), color[0], color[1], color[2], color[3]);
        gl.uniform4f(Some(&self.u_scale), width, height, 1.0, 1.0);
        gl.uniform4f(Some(&self.u_translation), x, y, 0.0, 0.0);
        gl.uniform1f(Some(&self.u_rotation), 0.0);

        gl.draw_elements_with_i32(GL::TRIANGLES, INDICES.len() as i32, GL::UNSIGNED_SHORT, 0);
    }

    pub fn bind(&self, gl: &GL) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertices));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices));

        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);
    }

    pub fn render_instances(&self, gl: &GL, instances: Vec<Instance>) {
        self.bind(gl);
        for instance in instances {
            gl.uniform4f(
                Some(&self.u_color),
                instance.color[0],
                instance.color[1],
                instance.color[2],
                instance.color[3],
            );
            gl.uniform4f(
                Some(&self.u_scale),
                instance.width,
                instance.height,
                1.0,
                1.0,
            );
            gl.uniform4f(Some(&self.u_translation), instance.x, instance.y, 0.0, 0.0);
            gl.uniform1f(Some(&self.u_rotation), instance.angle);
            gl.draw_elements_with_i32(GL::TRIANGLES, INDICES.len() as i32, GL::UNSIGNED_SHORT, 0);
        }
    }
}
