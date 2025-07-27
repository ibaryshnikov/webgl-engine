use std::fmt::Display;

use js_sys::{Error, Float32Array, Uint16Array, WebAssembly};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGlBuffer, WebGlRenderingContext};

use crate::rendering_context::{BufferKind, RenderingContext, get_buffer_type};

pub struct BufferStorage<T: Display + Sized> {
    kind: BufferKind,
    buffer: WebGlBuffer,
    data: Vec<T>,
}

impl<T: Display + Sized> BufferStorage<T> {
    pub fn new(ctx: &RenderingContext, kind: BufferKind, data: Vec<T>) -> Result<Self, Error> {
        let buffer = ctx.create_buffer()?;
        Ok(BufferStorage { kind, buffer, data })
    }

    pub fn bind(&self, ctx: &RenderingContext) {
        ctx.bind_buffer(self.kind, &self.buffer);
    }

    fn get_typed_array(&self) -> Result<JsValue, Error> {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        match self.kind {
            BufferKind::ArrayBuffer => {
                let start = self.data.as_ptr() as u32 / 4;
                let end = start + self.data.len() as u32;
                Ok(Float32Array::new(&memory_buffer)
                    .subarray(start, end)
                    .into())
            }
            BufferKind::ElementArrayBuffer => {
                let start = self.data.as_ptr() as u32 / 2;
                let end = start + self.data.len() as u32;
                Ok(Uint16Array::new(&memory_buffer).subarray(start, end).into())
            }
        }
    }

    pub fn write_to_graphics_card(&self, ctx: &RenderingContext) -> Result<(), Error> {
        let typed_array = self.get_typed_array()?;

        self.bind(ctx);
        let buffer_type = get_buffer_type(self.kind);
        ctx.gl.buffer_data_with_array_buffer_view(
            buffer_type,
            &typed_array.into(),
            WebGlRenderingContext::STATIC_DRAW,
        );
        ctx.clear_array_buffer();

        Ok(())
    }
}
