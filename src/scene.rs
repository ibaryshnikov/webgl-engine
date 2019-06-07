use std::cell::RefCell;
use std::ops::Drop;
use std::rc::Rc;

use js_sys::Date;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use crate::cube::{get_geometries, Cube};
use crate::dom_helpers::*;
use crate::program::Program;
use crate::rendering_context::RenderingContext;

type CanvasRef = Rc<RefCell<HtmlCanvasElement>>;

fn canvas_to_ref(canvas: HtmlCanvasElement) -> CanvasRef {
    Rc::new(RefCell::new(canvas))
}

#[wasm_bindgen]
pub struct Scene {
    canvas: CanvasRef,
    ctx: RenderingContext,
    program: Program,
    geometries: Vec<Cube>,
    last_update: f64,
}

impl Scene {
    pub fn get_ctx(&self) -> &RenderingContext {
        &self.ctx
    }

    pub fn get_program(&self) -> &Program {
        &self.program
    }

    fn clear_colors(&self) {
        let gl = &self.ctx.gl;

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);
        gl.enable(WebGlRenderingContext::DEPTH_TEST);
        gl.depth_func(WebGlRenderingContext::LEQUAL);

        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn use_program(&self) {
        self.ctx.use_program(&self.program.compiled);
    }

    fn set_initial_state(&mut self) {
        for geometry in &mut self.geometries {
            geometry.set_initial_state();
        }
    }
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Scene, JsValue> {
        let window = get_window()?;
        let document = get_document(&window)?;
        let body = get_body(&document)?;
        let canvas = create_canvas(&document)?;

        canvas.set_width(640);
        canvas.set_height(480);
        canvas.style().set_property("border", "1px solid black")?;

        body.append_child(&canvas)?;

        let gl = get_context(&canvas)?;
        let ctx = RenderingContext::new(gl);

        let vert_src = include_str!("shaders/vert.glsl");
        let frag_src = include_str!("shaders/frag.glsl");
        let program = Program::new(&ctx, vert_src, frag_src)?;

        let geometries = get_geometries(&ctx)?;

        let mut scene = Scene {
            ctx,
            program,
            geometries,
            last_update: Date::now(),
            canvas: canvas_to_ref(canvas),
        };

        scene.set_initial_state();

        Ok(scene)
    }

    pub fn update_state(&mut self) {
        let now = Date::now();
        let diff = now - self.last_update;
        self.last_update = now;

        let angle = diff as f32 / 1e3;

        for geometry in &mut self.geometries {
            geometry.update_state(angle);
        }
    }

    pub fn draw(&self) {
        self.clear_colors();
        for geometry in &self.geometries {
            geometry.draw(self);
        }
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        self.canvas.borrow().remove();
        // also remove event listeners
    }
}
