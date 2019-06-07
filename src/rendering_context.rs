use js_sys::Error;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};

enum ShaderKind {
    Vertex,
    Fragment,
}
#[derive(Clone, Copy)]
pub enum BufferKind {
    ArrayBuffer,
    ElementArrayBuffer,
}

fn get_shader_type(kind: ShaderKind) -> u32 {
    match kind {
        ShaderKind::Vertex => WebGlRenderingContext::VERTEX_SHADER,
        ShaderKind::Fragment => WebGlRenderingContext::FRAGMENT_SHADER,
    }
}

pub fn get_buffer_type(kind: BufferKind) -> u32 {
    match kind {
        BufferKind::ArrayBuffer => WebGlRenderingContext::ARRAY_BUFFER,
        BufferKind::ElementArrayBuffer => WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
    }
}

pub struct RenderingContext {
    pub gl: WebGlRenderingContext,
}

impl RenderingContext {
    pub fn new(gl: WebGlRenderingContext) -> Self {
        RenderingContext { gl }
    }

    pub fn bind_buffer(&self, kind: BufferKind, buffer: &WebGlBuffer) {
        let buffer_type = get_buffer_type(kind);
        self.gl.bind_buffer(buffer_type, Some(buffer));
    }

    pub fn clear_buffer(&self, kind: BufferKind) {
        let buffer_type = get_buffer_type(kind);
        self.gl.bind_buffer(buffer_type, None);
    }

    pub fn bind_array_buffer(&self, buffer: &WebGlBuffer) {
        self.bind_buffer(BufferKind::ArrayBuffer, buffer);
    }

    pub fn clear_array_buffer(&self) {
        self.clear_buffer(BufferKind::ArrayBuffer);
    }

    pub fn create_buffer(&self) -> Result<WebGlBuffer, Error> {
        self.gl
            .create_buffer()
            .ok_or_else(|| Error::new("Failed to create buffer"))
    }

    pub fn create_program(
        &self,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<WebGlProgram, Error> {
        let vertex_shader = self.compile_shader(ShaderKind::Vertex, vertex_src)?;
        let fragment_shader = self.compile_shader(ShaderKind::Fragment, fragment_src)?;
        self.link_program(vertex_shader, fragment_shader)
    }

    fn compile_shader(&self, kind: ShaderKind, source: &str) -> Result<WebGlShader, Error> {
        let gl = &self.gl;

        let shader_type = get_shader_type(kind);
        let shader = gl
            .create_shader(shader_type)
            .ok_or_else(|| Error::new("Unable to create shader object"))?;

        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(Error::new(&gl.get_shader_info_log(&shader).unwrap_or_else(
                || "Unknown error creating shader".to_owned(),
            )))
        }
    }

    fn link_program(
        &self,
        vertex_shader: WebGlShader,
        fragment_shader: WebGlShader,
    ) -> Result<WebGlProgram, Error> {
        let gl = &self.gl;

        let program = gl
            .create_program()
            .ok_or_else(|| Error::new("Unable to create shader program"))?;

        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(Error::new(
                &gl.get_program_info_log(&program)
                    .unwrap_or_else(|| "Unknown error creating shader program".to_owned()),
            ))
        }
    }

    pub fn get_attrib_location(&self, program: &WebGlProgram, name: &str) -> i32 {
        self.gl.get_attrib_location(program, name)
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGlProgram,
        name: &str,
    ) -> Result<WebGlUniformLocation, Error> {
        self.gl
            .get_uniform_location(program, name)
            .ok_or_else(|| Error::new(&format!("Can't get uniform location for {}", name)))
    }

    pub fn use_program(&self, program: &WebGlProgram) {
        self.gl.use_program(Some(program));
    }

    // pub fn clear_program(&self) {
    //     self.gl.use_program(None);
    // }
}
