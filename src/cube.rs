use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Unit, UnitQuaternion, Vector3};
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

use crate::buffer_storage::BufferStorage;
use crate::geometry::{get_colors, get_cube};
use crate::program::{AttributeKind, ProgramAttribute, UniformKind};
use crate::rendering_context::{BufferKind, RenderingContext};
use crate::scene::Scene;

struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Position {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Position { x, y, z }
    }
}

pub struct Cube {
    colors: Vec<f32>,
    indices_buffer: BufferStorage<u16>,
    vertices_buffer: BufferStorage<f32>,
    colors_buffer: BufferStorage<f32>,
    projection: Perspective3<f32>,
    model_view: Isometry3<f32>,
    position: Position,
}

impl Cube {
    fn enable_vertex_attrib_array(&self, ctx: &RenderingContext, attribute: &ProgramAttribute) {
        let options = &attribute.options;
        ctx.gl.vertex_attrib_pointer_with_i32(
            attribute.location as u32,
            options.size,
            options.data_type,
            options.normalized,
            options.stride,
            options.offset,
        );
        ctx.gl.enable_vertex_attrib_array(attribute.location as u32);
    }

    fn enable_attribute(&self, scene: &Scene, kind: AttributeKind) {
        match kind {
            AttributeKind::Vertex => {
                self.vertices_buffer.bind(scene.get_ctx());
                self.enable_vertex_attrib_array(
                    scene.get_ctx(),
                    &scene.get_program().attributes.vertices,
                );
            }
            AttributeKind::Color => {
                self.colors_buffer.bind(scene.get_ctx());
                self.enable_vertex_attrib_array(
                    scene.get_ctx(),
                    &scene.get_program().attributes.colors,
                );
            }
        }
    }

    fn set_uniform(&self, scene: &Scene, kind: UniformKind) {
        match kind {
            UniformKind::Projection => {
                let data = &self.projection.into_inner().data;
                scene.get_ctx().gl.uniform_matrix4fv_with_f32_array(
                    Some(&scene.get_program().uniform_locations.projection),
                    false,
                    data,
                );
            }
            UniformKind::ModelView => {
                let data = &self.model_view.to_homogeneous().data;
                scene.get_ctx().gl.uniform_matrix4fv_with_f32_array(
                    Some(&scene.get_program().uniform_locations.model_view),
                    false,
                    data,
                );
            }
        }
    }

    pub fn set_initial_state(&mut self) {
        let field_of_view = 45.0 * std::f32::consts::PI / 180.0; // in radians
        let width = 640.0;
        let height = 480.0;
        let aspect = width / height;
        let z_near = 0.1;
        let z_far = 500.0;

        self.projection = Perspective3::new(aspect, field_of_view, z_near, z_far);

        // Our object is translated along the x axis.
        let model = Isometry3::<f32>::new(Vector3::new(0.0, 0.0, 0.0), nalgebra::zero());

        // Our camera looks toward the point (1.0, 0.0, 0.0).
        // It is located at (0.0, 0.0, 1.0).
        let eye = Point3::<f32>::new(0.0, 0.0, 450.0);

        let Position { x, y, z } = self.position;
        let target = Point3::<f32>::new(x, y, z);
        let view = Isometry3::<f32>::look_at_rh(&eye, &target, &Vector3::y());

        // The combination of the model with the view is still an isometry.
        self.model_view = view * model;
    }

    pub fn update_state(&mut self, angle: f32) {
        let mut matrix = Isometry3::identity();

        let axis = Unit::new_normalize(Vector3::new(0.0, 0.0, 1.0));
        let rotation = UnitQuaternion::from_axis_angle(&axis, angle);
        matrix.append_rotation_mut(&rotation);

        let axis = Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0));
        let rotation = UnitQuaternion::from_axis_angle(&axis, angle);
        matrix.append_rotation_mut(&rotation);

        self.model_view *= matrix;
    }

    fn draw_elements(&self, scene: &Scene) {
        let vertex_count = 36;
        let data_type = WebGlRenderingContext::UNSIGNED_SHORT;
        let offset = 0;
        scene.get_ctx().gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            vertex_count,
            data_type,
            offset,
        );
    }

    pub fn draw(&self, scene: &Scene) {
        self.enable_attribute(scene, AttributeKind::Vertex);
        self.enable_attribute(scene, AttributeKind::Color);

        self.indices_buffer.bind(scene.get_ctx());
        scene.use_program();

        self.set_uniform(scene, UniformKind::Projection);
        self.set_uniform(scene, UniformKind::ModelView);

        self.draw_elements(scene);
        scene.get_ctx().clear_array_buffer();
    }
}

fn get_one_cube_geometry(ctx: &RenderingContext, position: Position) -> Result<Cube, JsValue> {
    let geometry = get_cube();
    let colors = get_colors();

    let vertices_buffer =
        BufferStorage::new(&ctx, BufferKind::ArrayBuffer, geometry.vertices.clone())?;
    vertices_buffer.write_to_graphics_card(&ctx)?;

    let colors_buffer = BufferStorage::new(&ctx, BufferKind::ArrayBuffer, colors.clone())?;
    colors_buffer.write_to_graphics_card(&ctx)?;

    let indices_buffer = BufferStorage::new(
        &ctx,
        BufferKind::ElementArrayBuffer,
        geometry.indices.clone(),
    )?;
    indices_buffer.write_to_graphics_card(&ctx)?;

    let projection = Perspective3::from_matrix_unchecked(Matrix4::zeros());
    let model_view = Isometry3::identity();

    Ok(Cube {
        colors,
        indices_buffer,
        vertices_buffer,
        colors_buffer,
        projection,
        model_view,
        position,
    })
}

fn make_position(x: i32, y: i32, z: i32) -> Position {
    Position::new(6.0 * x as f32, 6.0 * y as f32, 6.0 * z as f32)
}

pub fn get_geometries(ctx: &RenderingContext) -> Result<Vec<Cube>, JsValue> {
    let mut positions = vec![];
    let field_size = 30;
    let z = 0;
    for i in 0..field_size {
        for j in 0..field_size {
            match (i, j) {
                (0, 0) => positions.push(make_position(0, 0, z)),
                (x, 0) => {
                    positions.push(make_position(x, 0, z));
                    positions.push(make_position(-x, 0, z));
                }
                (0, y) => {
                    positions.push(make_position(0, y, z));
                    positions.push(make_position(0, -y, z));
                }
                (x, y) => {
                    positions.push(make_position(x, y, z));
                    positions.push(make_position(-x, y, z));
                    positions.push(make_position(x, -y, z));
                    positions.push(make_position(-x, -y, z));
                }
            }
        }
    }

    let mut results = vec![];
    for Position { x, y, z } in positions.iter() {
        results.push(get_one_cube_geometry(ctx, Position::new(*x, *y, *z))?);
    }

    Ok(results)
}
