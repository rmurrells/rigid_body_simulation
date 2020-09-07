use super::{
    camera::Camera,
    render_object_creator::RenderObjectCreator,
    screen_buffer::{
        Color, ScreenBuffer, ScreenBufferAccess, ScreenBufferTrait,
    },
};
use crate::{
    math::{matrix::Matrix3x3, polyhedron::Polyhedron, vector::Vector3d},
    mesh::Mesh,
};
use std::f64::consts::PI;

pub struct Draw3d {
    pub camera: Camera,
    screen_buffer: ScreenBuffer,
    render_object_creator: RenderObjectCreator,
}

impl Draw3d {
    pub fn new(window_size: (u32, u32)) -> Self {
        Self {
            camera: Camera::default(),
            screen_buffer: ScreenBuffer::new(window_size),
            render_object_creator: RenderObjectCreator::new(
                window_size,
                0.1,
                20000.,
                PI / 2.,
            ),
        }
    }

    pub fn get_simple_light_color(color: Color, light: f64) -> Color {
        let light = -(light - 1.) / 2.;
        Color::rgb(
            (color.r as f64 * light) as u8,
            (color.g as f64 * light) as u8,
            (color.b as f64 * light) as u8,
        )
    }
}

pub trait Draw3dAccess {
    fn draw_3d_access(&self) -> &Draw3d;
    fn draw_3d_access_mut(&mut self) -> &mut Draw3d;
}

pub trait Draw3dTrait: Draw3dAccess {
    fn camera_mut(&mut self) -> &mut Camera {
        &mut self.draw_3d_access_mut().camera
    }

    fn set_window_size(&mut self, window_size: (u32, u32)) {
        let draw_3d = self.draw_3d_access_mut();
        draw_3d.render_object_creator.set_window_size(window_size);
        draw_3d.screen_buffer.resize(window_size);
    }

    fn draw_aligned_cuboid(
        &mut self,
        min: &Vector3d,
        max: &Vector3d,
        color: Color,
    ) {
        let draw_3d = self.draw_3d_access_mut();
        draw_3d.draw_line(
            min,
            &Vector3d::new(max[0], min[1], min[2]),
            color,
            false,
        );
        draw_3d.draw_line(
            &Vector3d::new(max[0], min[1], min[2]),
            &Vector3d::new(max[0], max[1], min[2]),
            color,
            false,
        );
        draw_3d.draw_line(
            &Vector3d::new(max[0], max[1], min[2]),
            &Vector3d::new(min[0], max[1], min[2]),
            color,
            false,
        );
        draw_3d.draw_line(
            &Vector3d::new(min[0], max[1], min[2]),
            min,
            color,
            false,
        );

        draw_3d.draw_line(
            &Vector3d::new(min[0], min[1], max[2]),
            &Vector3d::new(max[0], min[1], max[2]),
            color,
            false,
        );
        draw_3d.draw_line(
            &Vector3d::new(max[0], min[1], max[2]),
            max,
            color,
            false,
        );
        draw_3d.draw_line(
            max,
            &Vector3d::new(min[0], max[1], max[2]),
            color,
            false,
        );
        draw_3d.draw_line(
            &Vector3d::new(min[0], max[1], max[2]),
            &Vector3d::new(min[0], min[1], max[2]),
            color,
            false,
        );

        draw_3d.draw_line(
            min,
            &Vector3d::new(min[0], min[1], max[2]),
            color,
            false,
        );
        draw_3d.draw_line(
            &Vector3d::new(max[0], min[1], min[2]),
            &Vector3d::new(max[0], min[1], max[2]),
            color,
            false,
        );
        draw_3d.draw_line(
            &Vector3d::new(max[0], max[1], min[2]),
            max,
            color,
            false,
        );
        draw_3d.draw_line(
            &Vector3d::new(min[0], max[1], min[2]),
            &Vector3d::new(min[0], max[1], max[2]),
            color,
            false,
        );
    }

    fn draw_line(
        &mut self,
        start: &Vector3d,
        end: &Vector3d,
        color: Color,
        in_front: bool,
    ) {
        let draw_3d = self.draw_3d_access_mut();
        if let Some(window_line) = &draw_3d
            .render_object_creator
            .get_window_line(start, end, &draw_3d.camera)
        {
            draw_3d.screen_buffer.draw_clipped_line(
                &window_line.finite_line_3d.start,
                &window_line.finite_line_3d.end,
                color,
                in_front,
            );
        }
    }

    fn draw_mesh(
        &mut self,
        mesh: &Mesh,
        world_position: &Vector3d,
        world_orientation: &Matrix3x3,
        color: Color,
    ) {
        let draw_3d = self.draw_3d_access_mut();
        for window_triangle in
            draw_3d.render_object_creator.get_window_triangles(
                &mesh,
                world_position,
                world_orientation,
                &draw_3d.camera,
            )
        {
            let color = Draw3d::get_simple_light_color(
                color,
                window_triangle.light_value,
            );
            draw_3d
                .screen_buffer
                .fill_triangle(&window_triangle.triangle_3d, color);
        }
    }

    fn draw_mesh_lines(
        &mut self,
        mesh: &Mesh,
        world_position: &Vector3d,
        world_orientation: &Matrix3x3,
        color: Color,
        in_front: bool,
    ) {
        let draw_3d = self.draw_3d_access_mut();
        for window_triangle in
            draw_3d.render_object_creator.get_window_triangles(
                &mesh,
                world_position,
                world_orientation,
                &draw_3d.camera,
            )
        {
            draw_3d.screen_buffer.draw_triangle_edges(
                &window_triangle.triangle_3d,
                color,
                in_front,
            );
        }
    }

    fn draw_polyhedron_edges(&mut self, polyhedron: &Polyhedron, color: Color) {
        let draw_3d = self.draw_3d_access_mut();
        let vertices = polyhedron.vertices();
        for edge in polyhedron.edges() {
            draw_3d.draw_line(
                &vertices[edge.start_index()],
                &vertices[edge.end_index()],
                color,
                false,
            );
        }
    }

    fn draw_position(&mut self, position: &Vector3d, color: Color) {
        let draw_3d = self.draw_3d_access_mut();
        if let Some(window_position) = &draw_3d
            .render_object_creator
            .get_window_pos(position, &draw_3d.camera)
        {
            draw_3d.screen_buffer.draw_position(window_position, color);
        }
    }
}

impl Draw3dAccess for Draw3d {
    fn draw_3d_access(&self) -> &Draw3d {
        self
    }
    fn draw_3d_access_mut(&mut self) -> &mut Draw3d {
        self
    }
}
impl Draw3dTrait for Draw3d {}

impl ScreenBufferAccess for Draw3d {
    fn screen_buffer_access(&self) -> &ScreenBuffer {
        &self.screen_buffer
    }
    fn screen_buffer_access_mut(&mut self) -> &mut ScreenBuffer {
        &mut self.screen_buffer
    }
}
impl ScreenBufferTrait for Draw3d {}
