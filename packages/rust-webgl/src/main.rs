use std::rc::Rc;

extern crate gleam;
use gleam::gl;
use gleam::gl::{GLenum};

mod webgl;
use webgl::{
    emscripten_GetProcAddress, emscripten_get_element_css_size,
    emscripten_webgl_create_context, emscripten_webgl_init_context_attributes,
    emscripten_webgl_make_context_current, EmscriptenWebGLContextAttributes,
};

const VERTICES: [f32; 6] = [
    0.0, 0.5, 
    0.5, -0.5, 
    -0.5, -0.5,
];

const INDICES: [u16; 3] = [0, 1, 2];

const VERT_CODE: &[&[u8]] = &[b"
    attribute vec3 coordinates;
    void main(void) {
        gl_Position = vec4(coordinates, 1.0);
    }"
];

const FRAG_CODE: &[&[u8]] = &[b"
    void main(void) {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1);
    }"
];

fn get_canvas_size() -> (u32, u32) {
    unsafe {
        let mut width = std::mem::uninitialized();
        let mut height = std::mem::uninitialized();
        emscripten_get_element_css_size(std::ptr::null(), &mut width, &mut height);
        (width as u32, height as u32)
    }
}

fn init_webgl() -> Rc<gl::Gl> {
    unsafe {
        let mut attributes: EmscriptenWebGLContextAttributes = std::mem::uninitialized();
        emscripten_webgl_init_context_attributes(&mut attributes);
        attributes.majorVersion = 2;

        let handle = emscripten_webgl_create_context(std::ptr::null(), &attributes);
        emscripten_webgl_make_context_current(handle);

        return gl::GlesFns::load_with(|addr| {
            let addr = std::ffi::CString::new(addr).unwrap();
            emscripten_GetProcAddress(addr.into_raw() as *const _) as *const _
        });
    }
}

fn create_buffer(gl: &Rc<gl::Gl>, vertices: Vec<f32>, indices: Vec<u16>) {
    let buffers = gl.gen_buffers(2);
    let vertex_buffer = buffers[0];
     
    gl.bind_buffer(gl::ARRAY_BUFFER, vertex_buffer);
    gl.buffer_data_untyped(
        gl::ARRAY_BUFFER,
        4 * vertices.len() as isize,
        vertices.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    let element_buffer = buffers[1];
    gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer);
    gl.buffer_data_untyped(
        gl::ELEMENT_ARRAY_BUFFER,
        2 * indices.len() as isize,
        indices.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );
}

fn create_shader(gl: &Rc<gl::Gl>, shader_type: GLenum, code: &[&[u8]]) -> u32 {
    let shader = gl.create_shader(shader_type);
    gl.shader_source(shader, code);
    gl.compile_shader(shader);

    let mut compiled = [0];
    unsafe {
        gl.get_shader_iv(shader, gl::COMPILE_STATUS, &mut compiled);
    }

    if compiled[0] == 0 {
        gl.delete_shader(shader);
    }

    return shader;
}

fn create_shaders(gl: &Rc<gl::Gl>, vert_code: &[&[u8]], frag_code: &[&[u8]]) {
    let vert_shader = create_shader(gl, gl::VERTEX_SHADER, vert_code);
    let frag_shader = create_shader(gl, gl::FRAGMENT_SHADER, frag_code);
    let program = gl.create_program();

    gl.attach_shader(program, vert_shader);
    gl.attach_shader(program, frag_shader);
    gl.link_program(program);
    gl.use_program(program);

    let coordinate_var = gl.get_attrib_location(program, "coordinates") as u32;
    gl.enable_vertex_attrib_array(coordinate_var);
    gl.vertex_attrib_pointer(coordinate_var, 2, gl::FLOAT, false, 0, 0);
}

fn draw(gl: &Rc<gl::Gl>) {
    let (width, height) = get_canvas_size();
    gl.viewport(0, 0, width as i32, height as i32);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(gl::COLOR_BUFFER_BIT);
    gl.draw_elements(gl::TRIANGLES, 3, gl::UNSIGNED_SHORT, 0);
}

pub fn main() {
    let gl = init_webgl();
    create_buffer(&gl, VERTICES.to_vec(), INDICES.to_vec());
    create_shaders(&gl, VERT_CODE, FRAG_CODE);
    draw(&gl);
}
