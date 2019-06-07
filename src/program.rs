use js_sys::Error;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

use crate::rendering_context::RenderingContext;

pub enum AttributeKind {
    Vertex,
    Color,
}
pub enum UniformKind {
    Projection,
    ModelView,
}

pub struct AttributeOptions {
    pub size: i32,
    pub data_type: u32,
    pub normalized: bool,
    pub stride: i32,
    pub offset: i32,
}

impl AttributeOptions {
    pub fn floats_with_size(size: i32) -> Self {
        AttributeOptions {
            size,
            data_type: WebGlRenderingContext::FLOAT,
            normalized: false,
            stride: 0,
            offset: 0,
        }
    }
}

pub struct ProgramAttribute {
    pub location: i32,
    pub options: AttributeOptions,
}

pub struct ProgramAttributesList {
    pub vertices: ProgramAttribute,
    pub colors: ProgramAttribute,
}

pub struct ProgramUniformsLocations {
    pub projection: WebGlUniformLocation,
    pub model_view: WebGlUniformLocation,
}

pub struct Program {
    pub compiled: WebGlProgram,
    pub attributes: ProgramAttributesList,
    pub uniform_locations: ProgramUniformsLocations,
}

impl Program {
    pub fn new(ctx: &RenderingContext, vert_src: &str, frag_src: &str) -> Result<Self, Error> {
        let compiled_program = ctx.create_program(vert_src, frag_src)?;

        let vertices = ProgramAttribute {
            location: ctx.get_attrib_location(&compiled_program, "aVertexPosition"),
            options: AttributeOptions::floats_with_size(3),
        };
        let colors = ProgramAttribute {
            location: ctx.get_attrib_location(&compiled_program, "aVertexColor"),
            options: AttributeOptions::floats_with_size(4),
        };

        let projection = ctx.get_uniform_location(&compiled_program, "uProjectionMatrix")?;
        let model_view = ctx.get_uniform_location(&compiled_program, "uModelViewMatrix")?;

        Ok(Program {
            compiled: compiled_program,
            attributes: ProgramAttributesList { vertices, colors },
            uniform_locations: ProgramUniformsLocations {
                projection,
                model_view,
            },
        })
    }
}
