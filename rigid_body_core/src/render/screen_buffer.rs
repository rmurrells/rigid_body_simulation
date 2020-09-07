use crate::math::{triangle::Triangle3d, vector::Vector3d};
use std::{cmp, mem};

pub const PIXEL_FORMAT: usize = 4;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct ScreenBuffer {
    window_size: (u32, u32),
    data: Vec<u8>,
    depth: Vec<f64>,
}

impl ScreenBuffer {
    pub fn new(window_size: (u32, u32)) -> Self {
        let window_len = (window_size.0 * window_size.1) as usize;
        Self {
            window_size,
            data: vec![255; window_len * PIXEL_FORMAT],
            depth: vec![0.; window_len],
        }
    }

    pub fn resize(&mut self, window_size: (u32, u32)) {
        *self = Self::new(window_size);
    }

    pub fn window_size(&self) -> (u32, u32) {
        self.window_size
    }

    pub fn draw_clipped_line(
        &mut self,
        start: &Vector3d,
        end: &Vector3d,
        color: Color,
        in_front: bool,
    ) {
        if start.is_nan() || end.is_nan() {
            println!(
                "screen_buffer draw_line - supplied NaN: {} {}",
                start, end,
            );
            return;
        }
        let mut draw_line_impl = |x0: i32,
                                  y0: i32,
                                  z0: f64,
                                  x1: i32,
                                  y1: i32,
                                  z1: f64,
                                  low: bool| {
            let (a0, b0, a1, b1) = if low {
                (x0, y0, x1, y1)
            } else {
                (y0, x0, y1, x1)
            };
            let da = a1 - a0;
            if da == 0 {
                return;
            }
            let mut db = b1 - b0;
            let dz = z1 - z0;
            let mut bi = 1;
            if db < 0 {
                bi = -1;
                db *= -1;
            }
            let mut d = 2 * db - da;
            let mut b = b0;
            let mut delta_z = 0.;
            let dz_step = 1. / da as f64;

            for a in a0..a1 {
                let depth_z = if in_front {
                    2.
                } else {
                    let depth_z = z0 + delta_z * dz;
                    delta_z += dz_step;
                    depth_z
                };
                if low {
                    self.fill_point(a as u32, b as u32, depth_z, color);
                } else {
                    self.fill_point(b as u32, a as u32, depth_z, color);
                }
                if d > 0 {
                    b += bi;
                    d -= 2 * da;
                }
                d += 2 * db;
            }
        };
        let x0 = start[0] as i32;
        let y0 = start[1] as i32;
        let z0 = start[2];
        let x1 = end[0] as i32;
        let y1 = end[1] as i32;
        let z1 = end[2];
        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                draw_line_impl(x1, y1, z1, x0, y0, z0, true)
            } else {
                draw_line_impl(x0, y0, z0, x1, y1, z1, true)
            }
        } else if y0 > y1 {
            draw_line_impl(x1, y1, z1, x0, y0, z0, false)
        } else {
            draw_line_impl(x0, y0, z0, x1, y1, z1, false)
        }
    }

    pub fn draw_position(&mut self, position: &Vector3d, color: Color) {
        self.fill_rect(
            (position[0] as u32 - 3, position[1] as u32 - 3),
            (position[0] as u32 + 3, position[1] as u32 + 3),
            position[2],
            color,
        );
    }

    pub fn draw_triangle_edges(
        &mut self,
        triangle: &Triangle3d,
        color: Color,
        in_front: bool,
    ) {
        let vertices = &triangle.vertices;
        self.draw_clipped_line(&vertices[0], &vertices[1], color, in_front);
        self.draw_clipped_line(&vertices[1], &vertices[2], color, in_front);
        self.draw_clipped_line(&vertices[2], &vertices[0], color, in_front);
    }

    pub fn fill_rect(
        &mut self,
        min: (u32, u32),
        max: (u32, u32),
        depth: f64,
        color: Color,
    ) {
        for y in min.1..max.1 {
            for x in min.0..max.0 {
                self.fill_point(x, y, depth, color);
            }
        }
    }

    pub fn fill_point(&mut self, x: u32, y: u32, depth: f64, color: Color) {
        if x > self.window_size.0 - 1 || y > self.window_size.1 - 1 {
            return;
        }
        let index = (x + y * self.window_size.0) as usize;
        let buffer_depth = &mut self.depth[index];
        if depth > *buffer_depth {
            *buffer_depth = depth;
            self.fill_pixel(index, color);
        }
    }

    pub fn fill_triangle(&mut self, triangle: &Triangle3d, color: Color) {
        let (vertex_1, vertex_2, vertex_3) =
            ScreenBuffer::get_y_sorted_vertices(triangle);
        let line_12 = Line::from_vertices(&vertex_1, &vertex_2);
        let line_13 = Line::from_vertices(&vertex_1, &vertex_3);
        let line_23 = Line::from_vertices(&vertex_2, &vertex_3);

        let window_size = self.window_size();
        let mut fill_half = |y_start: i32,
                             y_end: i32,
                             line_a: &Line,
                             line_b: &Line| {
            for y_step in y_start..y_end {
                let get_range = |line: &Line| {
                    (
                        (line.m * y_step as f64) as i32 + line.c,
                        line.m_depth * y_step as f64 + line.c_depth,
                    )
                };
                let (mut start_x, mut start_depth) = get_range(&line_a);
                let (mut end_x, mut end_depth) = get_range(&line_b);
                if end_x == start_x {
                    continue;
                }
                if end_x < start_x {
                    mem::swap(&mut start_x, &mut end_x);
                    mem::swap(&mut start_depth, &mut end_depth);
                }
                end_x = cmp::min(end_x + 1, window_size.0 as i32);

                let mut d = 0.;
                let d_step = 1. / (end_x - start_x) as f64;
                for x_step in start_x..end_x {
                    let depth = start_depth + d * (end_depth - start_depth);
                    self.fill_point(x_step as u32, y_step as u32, depth, color);
                    d += d_step;
                }
            }
        };
        fill_half(vertex_1.y, vertex_2.y, &line_12, &line_13);
        fill_half(vertex_2.y, vertex_3.y, &line_23, &line_13);
    }

    fn fill_pixel(&mut self, mut index: usize, color: Color) {
        index *= PIXEL_FORMAT;
        unsafe {
            *self.data.get_unchecked_mut(index) = color.r;
            *self.data.get_unchecked_mut(index + 1) = color.g;
            *self.data.get_unchecked_mut(index + 2) = color.b;
        }
    }

    fn get_y_sorted_vertices(
        triangle: &Triangle3d,
    ) -> (Vertex, Vertex, Vertex) {
        let get_vertex = |index: usize| {
            let v = &triangle.vertices[index];
            Vertex {
                x: v[0] as i32,
                y: v[1] as i32,
                z: v[2],
            }
        };
        let mut vertex_1 = get_vertex(0);
        let mut vertex_2 = get_vertex(1);
        let mut vertex_3 = get_vertex(2);
        if vertex_2.y < vertex_1.y {
            mem::swap(&mut vertex_2, &mut vertex_1);
        }
        if vertex_3.y < vertex_2.y {
            mem::swap(&mut vertex_3, &mut vertex_2);
        }
        if vertex_2.y < vertex_1.y {
            mem::swap(&mut vertex_2, &mut vertex_1);
        }
        (vertex_1, vertex_2, vertex_3)
    }
}

struct Line {
    m: f64,
    c: i32,
    m_depth: f64,
    c_depth: f64,
}

impl Line {
    fn from_vertices(vertex_a: &Vertex, vertex_b: &Vertex) -> Self {
        let dx_ba = vertex_b.x - vertex_a.x;
        let dy_ba = vertex_b.y - vertex_a.y;
        let dz_ba = vertex_b.z - vertex_a.z;
        if dy_ba == 0 {
            Self {
                m: 0.,
                c: 0,
                m_depth: 0.,
                c_depth: 0.,
            }
        } else {
            let m = dx_ba as f64 / dy_ba as f64;
            let m_depth = dz_ba / dy_ba as f64;
            Self {
                m,
                c: vertex_a.x - (m * vertex_a.y as f64) as i32,
                m_depth,
                c_depth: vertex_a.z - (m_depth * vertex_a.y as f64),
            }
        }
    }
}

struct Vertex {
    x: i32,
    y: i32,
    z: f64,
}

pub trait ScreenBufferAccess {
    fn screen_buffer_access(&self) -> &ScreenBuffer;
    fn screen_buffer_access_mut(&mut self) -> &mut ScreenBuffer;
}

pub trait ScreenBufferTrait: ScreenBufferAccess {
    fn clear(&mut self, color: Color) {
        let screen_buffer = self.screen_buffer_access_mut();
        for i in 0..screen_buffer.data.len() / PIXEL_FORMAT {
            screen_buffer.fill_pixel(i, color);
        }
        screen_buffer.depth.iter_mut().for_each(|e| *e = 0.);
    }

    fn pixel_buffer(&self) -> &[u8] {
        &self.screen_buffer_access().data
    }

    fn pixel_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.screen_buffer_access_mut().data
    }
}

impl ScreenBufferAccess for ScreenBuffer {
    fn screen_buffer_access(&self) -> &ScreenBuffer {
        self
    }
    fn screen_buffer_access_mut(&mut self) -> &mut ScreenBuffer {
        self
    }
}
impl ScreenBufferTrait for ScreenBuffer {}
