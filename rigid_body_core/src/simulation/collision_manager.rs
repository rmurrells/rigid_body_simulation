use super::{
    bounding_box_collision_manager::BoundingBoxCollisionManager,
    collision_table::{
        CollisionTable, Contact, Contacts, EdgeEdgeIndices, EdgeIndices,
        FaceIndices, VertexFaceIndices,
    },
    rigid_body::RigidBody,
};
use crate::math::{geometry, matrix_vector, vector::Vector3d};
use std::f64::{EPSILON, MAX};

pub use super::collision_table::SeparatingPlane;

const COLLISION_EPSILON: f64 = 1e-3;
const COR: f64 = 1.;

pub struct CollisionManager {
    pub debug: bool,
    bounding_box_collision_manager: BoundingBoxCollisionManager,
    collision_table: CollisionTable,
}

impl CollisionManager {
    pub fn new() -> Self {
        Self {
            debug: false,
            bounding_box_collision_manager: BoundingBoxCollisionManager::new(
                COLLISION_EPSILON,
            ),
            collision_table: CollisionTable::new(),
        }
    }

    #[allow(dead_code)]
    pub fn is_bounding_box_colliding(&self, i: usize) -> bool {
        for j in 0..self.collision_table.len() {
            if self.collision_table.get(i, j).bounding_box_collision() {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn is_colliding(&self, i: usize) -> bool {
        for j in 0..self.collision_table.len() {
            if self.collision_table.get(i, j).colliding {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn collision_table(&self) -> &CollisionTable {
        &self.collision_table
    }

    pub fn collide_simple(&mut self, rigid_bodies: &mut [RigidBody]) {
        self.bounding_box_collision_manager
            .update(rigid_bodies, &mut self.collision_table);
        self.collision_table.reset_colliding();
        for j in 0..rigid_bodies.len() {
            for i in j + 1..rigid_bodies.len() {
                if rigid_bodies[i].is_immovable()
                    && rigid_bodies[j].is_immovable()
                {
                    continue;
                }
                if !self.collision_table.get(i, j).bounding_box_collision() {
                    continue;
                }
                if !self.check_for_separating_plane(i, j, rigid_bodies) {
                    self.collision_table.get_mut(i, j).colliding = true;
                    self.handle_collision_simple(i, j, rigid_bodies);
                    self.bounding_box_collision_manager
                        .update(rigid_bodies, &mut self.collision_table);
                }
            }
        }
    }

    pub fn generate(&mut self, rigid_bodies: &[RigidBody]) {
        self.collision_table.generate(rigid_bodies.len());
        self.bounding_box_collision_manager
            .generate(rigid_bodies, &mut self.collision_table);
        for j in 0..rigid_bodies.len() {
            for i in j + 1..rigid_bodies.len() {
                if rigid_bodies[i].is_immovable()
                    && rigid_bodies[j].is_immovable()
                {
                    continue;
                }
                self.check_for_separating_plane(i, j, rigid_bodies);
            }
        }
    }

    fn check_for_separating_plane(
        &mut self,
        rigid_body_1_index: usize,
        rigid_body_2_index: usize,
        rigid_bodies: &[RigidBody],
    ) -> bool {
        let separating_plane = &mut self
            .collision_table
            .get_mut(rigid_body_1_index, rigid_body_2_index)
            .separating_plane;
        match &separating_plane {
            SeparatingPlane::Face { face_indices } => {
                if Self::face_is_separating_plane(face_indices, rigid_bodies) {
                    return true;
                }
                *separating_plane = SeparatingPlane::None;
            }
            SeparatingPlane::Edge { edge_indices } => {
                if Self::edges_make_separating_plane(edge_indices, rigid_bodies)
                {
                    return true;
                }
                *separating_plane = SeparatingPlane::None;
            }
            SeparatingPlane::None => (),
        }
        if Self::separating_plane_face_search(
            rigid_body_1_index,
            rigid_body_2_index,
            rigid_bodies,
            separating_plane,
        ) || Self::separating_plane_face_search(
            rigid_body_2_index,
            rigid_body_1_index,
            rigid_bodies,
            separating_plane,
        ) {
            return true;
        }
        Self::separating_plane_edge_search(
            rigid_body_1_index,
            rigid_body_2_index,
            rigid_bodies,
            separating_plane,
        )
    }

    fn contact_force(
        contact: &Contact,
        rigid_bodies: &mut [RigidBody],
    ) -> bool {
        let position_rigid_body_index;
        let normal_rigid_body_index;
        let position;
        let normal;
        match contact {
            Contact::VertexFace {
                vertex_face_indices,
            } => {
                position_rigid_body_index =
                    vertex_face_indices.vertex_rigid_body;
                position = &rigid_bodies[position_rigid_body_index]
                    .polyhedron_world()
                    .vertices()[vertex_face_indices.vertex];
                normal_rigid_body_index = vertex_face_indices.face_rigid_body;
                normal = rigid_bodies[normal_rigid_body_index]
                    .polyhedron_world()
                    .faces()[vertex_face_indices.face]
                    .direction();
            }
            Contact::EdgeEdge { edge_edge_indices } => {
                position_rigid_body_index = edge_edge_indices.other_rigid_body;
                position = &edge_edge_indices.contact_position;
                normal_rigid_body_index = edge_edge_indices.plane_rigid_body;
                normal = &edge_edge_indices.plane_direction;
            }
        }
        let position_rigid_body = &rigid_bodies[position_rigid_body_index];
        let normal_rigid_body = &rigid_bodies[normal_rigid_body_index];

        let rel_com_position = position.sub(&position_rigid_body.position);
        let rel_com_normal = position.sub(&normal_rigid_body.position);

        let position_vel = position_rigid_body
            .angular_velocity()
            .cross(&rel_com_position)
            .add(position_rigid_body.velocity());
        let normal_vel = normal_rigid_body
            .angular_velocity()
            .cross(&rel_com_normal)
            .add(normal_rigid_body.velocity());

        let rel_vel = position_vel.sub(&normal_vel).dot(normal);
        if rel_vel >= 0. {
            return false;
        }

        let get_den_term = |rigid_body: &RigidBody, rel_com: &Vector3d| {
            rigid_body.mass_inv()
                + matrix_vector::mult_3(
                    &rigid_body.inertia_inv(),
                    &rel_com.cross(normal),
                )
                .cross(rel_com)
                .dot(normal)
        };
        let impulse_mag = (-(1. + COR) * rel_vel)
            / (get_den_term(position_rigid_body, &rel_com_position)
                + get_den_term(normal_rigid_body, &rel_com_normal));
        let impulse = normal.scale(impulse_mag);

        let position_rigid_body = &mut rigid_bodies[position_rigid_body_index];
        position_rigid_body.momentum.add_assign(&impulse);
        position_rigid_body
            .angular_momentum
            .add_assign(&rel_com_position.cross(&impulse));
        position_rigid_body.update_velocity();
        position_rigid_body.update_angular_velocity();

        let normal_rigid_body = &mut rigid_bodies[normal_rigid_body_index];
        normal_rigid_body.momentum.sub_assign(&impulse);
        normal_rigid_body
            .angular_momentum
            .sub_assign(&rel_com_normal.cross(&impulse));
        normal_rigid_body.update_velocity();
        normal_rigid_body.update_angular_velocity();

        true
    }

    fn contact_search(
        separating_plane: &SeparatingPlane,
        rigid_bodies: &[RigidBody],
        mode: &mut Mode,
    ) {
        if let Mode::Contacts { contacts } = mode {
            contacts.clear();
        }
        let separating_rigid_body_index;
        let separating_vertices;
        let separating_plane_pos;
        let store_dir;
        let separating_plane_dir;

        let other_rigid_body_index;

        match separating_plane {
            SeparatingPlane::Face { face_indices } => {
                separating_rigid_body_index = face_indices.face_rigid_body;
                let separating_polyhedron = rigid_bodies
                    [face_indices.face_rigid_body]
                    .polyhedron_world();
                separating_vertices = separating_polyhedron.vertices();
                separating_plane_pos =
                    &separating_vertices[face_indices.face_position];
                separating_plane_dir = separating_polyhedron.faces()
                    [face_indices.face]
                    .direction();

                other_rigid_body_index = face_indices.other_rigid_body;

                for vertex in 0..rigid_bodies[other_rigid_body_index]
                    .polyhedron_world()
                    .vertices()
                    .len()
                {
                    Self::vertex_face_dist_check(
                        &VertexFaceIndices {
                            vertex_rigid_body: other_rigid_body_index,
                            vertex,
                            face_rigid_body: separating_rigid_body_index,
                            face: face_indices.face,
                        },
                        rigid_bodies,
                        mode,
                    );
                }
            }
            SeparatingPlane::Edge { edge_indices } => {
                separating_rigid_body_index = edge_indices.plane_rigid_body;
                let separating_polyhedron = rigid_bodies
                    [separating_rigid_body_index]
                    .polyhedron_world();
                separating_vertices = separating_polyhedron.vertices();
                separating_plane_pos =
                    &separating_vertices[edge_indices.plane_position];
                store_dir = edge_indices
                    .plane_direction(rigid_bodies)
                    .expect("plane_direction");
                separating_plane_dir = &store_dir;

                other_rigid_body_index = edge_indices.other_rigid_body;
            }
            SeparatingPlane::None => unreachable!(),
        }
        Self::vertices_on_faces(
            separating_rigid_body_index,
            &Self::plane_coincident_vertices(
                separating_vertices,
                separating_plane_pos,
                separating_plane_dir,
            ),
            other_rigid_body_index,
            rigid_bodies,
            mode,
        );
        Self::edge_edge_dist(
            separating_rigid_body_index,
            separating_plane_pos,
            separating_plane_dir,
            other_rigid_body_index,
            rigid_bodies,
            mode,
        );
    }

    fn edge_edge_dist(
        plane_rigid_body_index: usize,
        plane_pos: &Vector3d,
        plane_dir: &Vector3d,
        other_rigid_body_index: usize,
        rigid_bodies: &[RigidBody],
        mode: &mut Mode,
    ) {
        let rigid_body = &rigid_bodies[plane_rigid_body_index];
        let polyhedron = rigid_body.polyhedron_world();
        let vertices = polyhedron.vertices();
        let edges = polyhedron.edges();

        let mut plane_coincident_edges = Vec::with_capacity(edges.len());
        for (edge_index, edge) in edges.iter().enumerate() {
            if geometry::pos_raw_plane_dist(
                &vertices[edge.start_index()],
                plane_pos,
                plane_dir,
            ) <= COLLISION_EPSILON
                || geometry::pos_raw_plane_dist(
                    &vertices[edge.end_index()],
                    plane_pos,
                    plane_dir,
                ) <= COLLISION_EPSILON
            {
                plane_coincident_edges.push(edge_index);
            }
        }

        let other_rigid_body = &rigid_bodies[other_rigid_body_index];
        let other_polyhedron = other_rigid_body.polyhedron_world();
        let other_vertices = other_polyhedron.vertices();
        let other_edges = other_polyhedron.edges();
        for edge_index in &plane_coincident_edges {
            let edge = &edges[*edge_index];
            let edge_start = &vertices[edge.start_index()];
            let edge_end = &vertices[edge.end_index()];
            for (i, other_edge) in other_edges.iter().enumerate() {
                let (contact_position, _, dist_sq) =
                    geometry::raw_finite_line_closest_dist_sq(
                        &other_vertices[other_edge.start_index()],
                        &other_vertices[other_edge.end_index()],
                        edge_start,
                        edge_end,
                    );
                let ip = dist_sq.sqrt();
                match mode {
                    Mode::ClosestDist { dist } => **dist = dist.min(ip),
                    Mode::Contacts { contacts } => {
                        if ip <= COLLISION_EPSILON {
                            let mut plane_direction =
                                edge.direction().cross(other_edge.direction());
                            if plane_direction.is_zero() {
                                plane_direction = *edge.direction();
                            }
                            if geometry::pos_raw_plane_signed_dist(
                                &rigid_body.position,
                                edge_start,
                                &plane_direction,
                            ) > 0.
                            {
                                plane_direction.scale_assign(-1.);
                            }
                            plane_direction.normalize();
                            contacts.push(Contact::EdgeEdge {
                                edge_edge_indices: EdgeEdgeIndices {
                                    plane_rigid_body: plane_rigid_body_index,
                                    plane_edge: *edge_index,
                                    other_rigid_body: other_rigid_body_index,
                                    other_edge: i,
                                    contact_position,
                                    plane_direction,
                                },
                            });
                        }
                    }
                }
            }
        }
    }

    fn edges_make_separating_plane(
        edge_indices: &EdgeIndices,
        rigid_bodies: &[RigidBody],
    ) -> bool {
        let plane_polyhedron =
            rigid_bodies[edge_indices.plane_rigid_body].polyhedron_world();
        let plane_vertices = plane_polyhedron.vertices();
        let plane_edge = &plane_polyhedron.edges()[edge_indices.plane_edge];
        let plane_edge_start = plane_edge.start_index();
        let plane_edge_end = plane_edge.end_index();
        let plane_position = &plane_vertices[edge_indices.plane_position];
        if let Some(plane_direction) =
            &edge_indices.plane_direction(rigid_bodies)
        {
            for (i, plane_vertex) in plane_vertices.iter().enumerate() {
                if i == plane_edge_start || i == plane_edge_end {
                    continue;
                }
                if geometry::pos_raw_plane_signed_dist(
                    plane_vertex,
                    plane_position,
                    plane_direction,
                ) > 0.
                {
                    return false;
                }
            }
            for other_vertex in rigid_bodies[edge_indices.other_rigid_body]
                .polyhedron_world()
                .vertices()
            {
                if geometry::pos_raw_plane_signed_dist(
                    other_vertex,
                    plane_position,
                    plane_direction,
                ) <= 0.
                {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn face_is_separating_plane(
        face_indices: &FaceIndices,
        rigid_bodies: &[RigidBody],
    ) -> bool {
        let face_polyhedron =
            rigid_bodies[face_indices.face_rigid_body].polyhedron_world();
        for other_vertex in rigid_bodies[face_indices.other_rigid_body]
            .polyhedron_world()
            .vertices()
        {
            if geometry::pos_raw_plane_signed_dist(
                other_vertex,
                &face_polyhedron.vertices()[face_indices.face_position],
                face_polyhedron.faces()[face_indices.face].direction(),
            ) <= 0.
            {
                return false;
            }
        }
        true
    }

    fn plane_coincident_vertices(
        vertices: &[Vector3d],
        plane_pos: &Vector3d,
        plane_dir: &Vector3d,
    ) -> Vec<usize> {
        let mut ret = Vec::with_capacity(vertices.len());
        for (vertex_index, vertex) in vertices.iter().enumerate() {
            if geometry::pos_raw_plane_dist(vertex, plane_pos, plane_dir)
                <= COLLISION_EPSILON
            {
                ret.push(vertex_index);
            }
        }
        ret
    }

    fn separating_plane_edge_search(
        rigid_body_1_index: usize,
        rigid_body_2_index: usize,
        rigid_bodies: &[RigidBody],
        separating_plane: &mut SeparatingPlane,
    ) -> bool {
        let edges_1 =
            rigid_bodies[rigid_body_1_index].polyhedron_world().edges();
        let edges_2 =
            rigid_bodies[rigid_body_2_index].polyhedron_world().edges();

        for (edge_1_index, edge_1) in edges_1.iter().enumerate() {
            for (edge_2_index, edge_2) in edges_2.iter().enumerate() {
                let mut edge_indices = EdgeIndices {
                    plane_rigid_body: rigid_body_1_index,
                    plane_edge: edge_1_index,
                    plane_position: edge_1.start_index(),
                    other_rigid_body: rigid_body_2_index,
                    other_edge: edge_2_index,
                };
                if Self::edges_make_separating_plane(
                    &edge_indices,
                    rigid_bodies,
                ) {
                    *separating_plane = SeparatingPlane::Edge { edge_indices };
                    return true;
                }

                edge_indices = EdgeIndices {
                    plane_rigid_body: rigid_body_2_index,
                    plane_edge: edge_2_index,
                    plane_position: edge_2.start_index(),
                    other_rigid_body: rigid_body_1_index,
                    other_edge: edge_1_index,
                };
                if Self::edges_make_separating_plane(
                    &edge_indices,
                    rigid_bodies,
                ) {
                    *separating_plane = SeparatingPlane::Edge { edge_indices };
                    return true;
                }
            }
        }
        false
    }

    fn separating_plane_face_search(
        rigid_body_1_index: usize,
        rigid_body_2_index: usize,
        rigid_bodies: &[RigidBody],
        separating_plane: &mut SeparatingPlane,
    ) -> bool {
        for (face_index, face) in rigid_bodies[rigid_body_1_index]
            .polyhedron_world()
            .faces()
            .iter()
            .enumerate()
        {
            let face_indices = FaceIndices {
                face_rigid_body: rigid_body_1_index,
                face: face_index,
                face_position: face.vertex_indices()[0],
                other_rigid_body: rigid_body_2_index,
            };
            if !Self::face_is_separating_plane(&face_indices, rigid_bodies) {
                continue;
            }
            *separating_plane = SeparatingPlane::Face { face_indices };
            return true;
        }
        false
    }

    fn vertex_face_dist_check(
        vertex_face_indices: &VertexFaceIndices,
        rigid_bodies: &[RigidBody],
        mode: &mut Mode,
    ) -> bool {
        let vertex = &rigid_bodies[vertex_face_indices.vertex_rigid_body]
            .polyhedron_world()
            .vertices()[vertex_face_indices.vertex];
        let face_polyhedron = rigid_bodies[vertex_face_indices.face_rigid_body]
            .polyhedron_world();
        let face_vertices = face_polyhedron.vertices();
        let face = &face_polyhedron.faces()[vertex_face_indices.face];

        for enclosing_plane in
            &face.enclosing_planes(face_vertices, face_polyhedron.edges())
        {
            if geometry::pos_raw_plane_signed_dist(
                vertex,
                &face_vertices[enclosing_plane.vertex_index()],
                enclosing_plane.direction(),
            ) >= 0.
            {
                return false;
            }
        }
        let signed_dist = geometry::pos_raw_plane_signed_dist(
            vertex,
            &face_vertices[face.vertex_indices()[0]],
            face.direction(),
        );
        if signed_dist > 0. {
            match mode {
                Mode::ClosestDist { dist } => **dist = dist.min(signed_dist),
                Mode::Contacts { contacts } => {
                    if signed_dist <= COLLISION_EPSILON {
                        contacts.push(Contact::VertexFace {
                            vertex_face_indices: *vertex_face_indices,
                        });
                    }
                }
            }
            true
        } else {
            false
        }
    }

    fn vertices_on_faces(
        vertex_rigid_body: usize,
        vertex_indices: &[usize],
        face_rigid_body: usize,
        rigid_bodies: &[RigidBody],
        mode: &mut Mode,
    ) {
        let faces = rigid_bodies[face_rigid_body].polyhedron_world().faces();
        'sf: for vertex_index in vertex_indices {
            for face in 0..faces.len() {
                if Self::vertex_face_dist_check(
                    &VertexFaceIndices {
                        vertex_rigid_body,
                        vertex: *vertex_index,
                        face_rigid_body,
                        face,
                    },
                    rigid_bodies,
                    mode,
                ) {
                    continue 'sf;
                }
            }
        }
    }

    fn handle_collision_simple(
        &mut self,
        rigid_body_1_index: usize,
        rigid_body_2_index: usize,
        rigid_bodies: &mut [RigidBody],
    ) {
        fn get_h_extent(rigid_body: &RigidBody) -> f64 {
            let bounding_box = rigid_body.bounding_box();
            let mut ret = 0.;
            for i in 0..3 {
                let dist_axis = bounding_box[1][i] - bounding_box[0][i]
                    + COLLISION_EPSILON * 2.;
                ret += dist_axis * dist_axis;
            }
            ret.sqrt() / 2.
        }
        let rigid_body_1 = &rigid_bodies[rigid_body_1_index];
        let rigid_body_2 = &rigid_bodies[rigid_body_2_index];
        let separating = rigid_body_2.position.sub(&rigid_body_1.position);
        let extent = get_h_extent(rigid_body_1) + get_h_extent(rigid_body_2);

        if separating.is_zero() {
            self.de_penetrate_dir(
                &Vector3d::new(1., 0., 0.),
                extent,
                rigid_body_1_index,
                rigid_body_2_index,
                rigid_bodies,
            );
        } else {
            self.de_penetrate_dir(
                &separating.normal(),
                extent - separating.mag(),
                rigid_body_1_index,
                rigid_body_2_index,
                rigid_bodies,
            );
        }
        let collision_status = self
            .collision_table
            .get_mut(rigid_body_1_index, rigid_body_2_index);
        Self::contact_search(
            &collision_status.separating_plane,
            rigid_bodies,
            &mut Mode::Contacts {
                contacts: &mut collision_status.contacts,
            },
        );
        if self.debug && collision_status.contacts.is_empty() {
            println!("closest_distance - Contacts",);
        }
        Self::contact_forces_simple(&collision_status.contacts, rigid_bodies);
    }

    fn de_penetrate_dir(
        &mut self,
        separating_dir: &Vector3d,
        mut bisect: f64,
        rigid_body_1_index: usize,
        rigid_body_2_index: usize,
        rigid_bodies: &mut [RigidBody],
    ) {
        let mass_inv_1 = rigid_bodies[rigid_body_1_index].mass_inv();
        let mass_inv_2 = rigid_bodies[rigid_body_2_index].mass_inv();
        let mass_inv_tot = mass_inv_1 + mass_inv_2;
        let mass_ratio_1 = mass_inv_1 / mass_inv_tot;
        let mass_ratio_2 = mass_inv_2 / mass_inv_tot;
        let mut dist = MAX;
        while dist >= COLLISION_EPSILON {
            if bisect.abs() < EPSILON {
                if self.debug {
                    println!(
                        "de_penetrate_dir failiure - bisect, uids: ({} {})",
                        rigid_bodies[rigid_body_1_index].uid(),
                        rigid_bodies[rigid_body_2_index].uid(),
                    );
                }
                return;
            }
            let separation = separating_dir.scale(bisect);
            let rigid_body_1 = &mut rigid_bodies[rigid_body_1_index];
            rigid_body_1
                .position
                .sub_assign(&separation.scale(mass_ratio_1));
            rigid_body_1.update_geometry();
            let rigid_body_2 = &mut rigid_bodies[rigid_body_2_index];
            rigid_body_2
                .position
                .add_assign(&separation.scale(mass_ratio_2));
            rigid_body_2.update_geometry();

            if self.check_for_separating_plane(
                rigid_body_1_index,
                rigid_body_2_index,
                rigid_bodies,
            ) {
                Self::contact_search(
                    &self
                        .collision_table
                        .get(rigid_body_1_index, rigid_body_2_index)
                        .separating_plane,
                    rigid_bodies,
                    &mut Mode::ClosestDist { dist: &mut dist },
                );
                if dist == MAX {
                    if self.debug {
                        println!(
                            "de_penetrate_dir failiure - plane, uids: ({} {})",
                            rigid_bodies[rigid_body_1_index].uid(),
                            rigid_bodies[rigid_body_2_index].uid(),
                        );
                    }
                    return;
                }
                bisect = bisect.copysign(-1.);
            } else {
                if dist == MAX {
                    if self.debug {
                        println!(
			    "de_penetrate_dir failiure - no plane, uids: ({} {})",
			    rigid_bodies[rigid_body_1_index].uid(),
			    rigid_bodies[rigid_body_2_index].uid(),
			);
                    }
                    return;
                }
                bisect = bisect.copysign(1.);
            }
            bisect /= 2.;
        }
    }

    fn contact_forces_simple(
        contacts: &[Contact],
        rigid_bodies: &mut [RigidBody],
    ) {
        for contact in contacts {
            Self::contact_force(contact, rigid_bodies);
        }
    }
}

impl Default for CollisionManager {
    fn default() -> Self {
        Self::new()
    }
}

enum Mode<'a> {
    ClosestDist { dist: &'a mut f64 },
    Contacts { contacts: &'a mut Contacts },
}
