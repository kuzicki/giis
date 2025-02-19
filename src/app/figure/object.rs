use super::{Drawable, Figure, Selectable, Transformable};
use eframe::egui::{Color32, InputState, Key, Pos2, Shape, Stroke};
use nalgebra::{Matrix4, Rotation3, Vector3, Vector4};

pub struct Object {
    selected: bool,
    vertices: Vec<Vector3<f32>>,
    indices: Vec<u32>,
    rotation: Vector3<f32>,
    rotation_speed: Vector3<f32>,
    screen_width: f32,
    screen_height: f32,
    fov: f32,
    translation: Vector3<f32>,
    mirror_x: bool,
    mirror_y: bool,
    mirror_z: bool,
    scale: Vector3<f32>,
}

impl Object {
    pub fn new(file_path: &str, center: Pos2) -> Self {
        let (models, _) = match tobj::load_obj(file_path, &tobj::LoadOptions::default()) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error: {}", e);
                return Self {
                    vertices: vec![],
                    indices: vec![],
                    selected: false,
                    rotation: Vector3::new(0.0, 0.0, 0.0),
                    rotation_speed: Vector3::new(1.0, 1.0, 1.0),
                    screen_width: 1536.0,
                    screen_height: 800.8,
                    fov: 90.0,
                    translation: Vector3::new(0.0, 0.0, 0.0),
                    mirror_x: false,
                    mirror_y: false,
                    mirror_z: false,
                    scale: Vector3::new(1.0, 1.0, 1.0),
                };
            }
        };

        let mut vertices: Vec<Vector3<f32>> = vec![];
        let mut indices = vec![];

        let scale_factor = 0.01;
        for vertex in &mut vertices {
            vertex.x *= scale_factor;
            vertex.y *= scale_factor;
            vertex.z *= scale_factor;
        }

        for model in models.iter() {
            let mesh = &model.mesh;
            for i in 0..mesh.positions.len() / 3 {
                let x = mesh.positions[i * 3];
                let y = mesh.positions[i * 3 + 1];
                let z = mesh.positions[i * 3 + 2];
                vertices.push(Vector3::new(x, y, z));
            }
            indices.extend(&mesh.indices);
        }

        Self {
            vertices,
            indices,
            selected: false,
            rotation: Vector3::new(0.0, 0.0, 0.0),
            rotation_speed: Vector3::new(1.0, 1.0, 1.0),
            screen_width: 1536.0,
            screen_height: 800.8,
            fov: 90.0,
            translation: Vector3::new(0.0, 0.0, 0.0),
            mirror_x: false,
            mirror_y: false,
            mirror_z: false,
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    fn get_mirror_matrix(&self) -> Matrix4<f32> {
        let mut mirror_matrix = Matrix4::identity();

        if self.mirror_x {
            mirror_matrix[(0, 0)] = -1.0;
        }
        if self.mirror_y {
            mirror_matrix[(1, 1)] = -1.0;
        }
        if self.mirror_z {
            mirror_matrix[(2, 2)] = -1.0;
        }

        mirror_matrix
    }

    fn get_scale_matrix(&self) -> Matrix4<f32> {
        let mut res = Matrix4::identity();
        for i in 0..self.scale.len() {
            res[(i, i)] = self.scale[i].clone();
        }

        res
    }

    fn get_rotation_matrix(&self) -> Matrix4<f32> {
        let rotation =
            Rotation3::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        rotation.to_homogeneous()
    }

    fn get_tranlation_matrix(&self) -> Matrix4<f32> {
        let mut res = Matrix4::identity();
        res.fixed_view_mut::<3, 1>(0, 3).copy_from(&self.translation);
        res
    }

    fn get_transform_matrix(&self) -> Matrix4<f32> {
        let rotation_matrix = self.get_rotation_matrix();
        let translation_matrix = self.get_tranlation_matrix();
        let scale_matrix = self.get_scale_matrix();
        let mirror_matrix = self.get_mirror_matrix();

        return translation_matrix * rotation_matrix * scale_matrix * mirror_matrix;
    }

    fn get_outline_color(&self) -> Stroke {
        if self.selected {
            Stroke::new(2.0, Color32::GREEN)
        } else {
            Stroke::new(2.0, Color32::BLACK)
        }
    }
}

impl Figure for Object {
    fn as_transformable(&self) -> Option<&dyn Transformable> {
        Some(self)
    }

    fn as_transformable_mut(&mut self) -> Option<&mut dyn Transformable> {
        Some(self)
    }

    fn as_selectable(&self) -> Option<&dyn Selectable> {
        Some(self)
    }

    fn as_selectable_mut(&mut self) -> Option<&mut dyn Selectable> {
        Some(self)
    }
}

impl Drawable for Object {
    fn draw(&self, painter: &eframe::egui::Painter) {
        let transform_matrix = self.get_transform_matrix();

        let perspective_matrix = get_perspective_matrix(
            self.fov,
            self.screen_width / self.screen_height,
            0.1,                                   
            10000000.0,                            
        );

        let transformed_vertices: Vec<Vector3<f32>> = self
            .vertices
            .iter()
            .map(|v| {
                let homogeneous_vertex = transform_matrix * Vector4::new(v.x, v.y, v.z, 1.0);
                let transformed = perspective_matrix * homogeneous_vertex;

                Vector3::new(
                    transformed.x / transformed.w,
                    transformed.y / transformed.w,
                    transformed.z / transformed.w,
                )
            })
            .collect();

        let mut points_2d = Vec::new();
        for vertex in &transformed_vertices {
            let coord = world_to_screen(*vertex, self.screen_width, self.screen_height);
            points_2d.push(coord);
        }

        for chunk in self.indices.chunks(3) {
            let p1 = points_2d[chunk[0] as usize];
            let p2 = points_2d[chunk[1] as usize];
            let p3 = points_2d[chunk[2] as usize];
            let stroke = self.get_outline_color();
            painter.add(Shape::line_segment([p1, p2], stroke));
            painter.add(Shape::line_segment([p2, p3], stroke));
            painter.add(Shape::line_segment([p3, p1], stroke));
        }
    }
}

impl Selectable for Object {
    fn select(&mut self) {
        self.selected = true;
    }

    fn deselect(&mut self) {
        self.selected = false;
    }

    fn hit_test(&self, pos: Pos2) -> bool {
        let transform_matrix = self.get_transform_matrix();

        let perspective_matrix = get_perspective_matrix(
            self.fov,
            self.screen_width / self.screen_height,
            0.1,                                    
            10000000.0,                             
        );

        let mut points_2d = Vec::new();
        for vertex in &self.vertices {
            let homogeneous_vertex =
                transform_matrix * Vector4::new(vertex.x, vertex.y, vertex.z, 1.0);

            let transformed = perspective_matrix * homogeneous_vertex;

            let projected = Vector3::new(
                transformed.x / transformed.w,
                transformed.y / transformed.w,
                transformed.z / transformed.w,
            );

            points_2d.push(world_to_screen(
                projected,
                self.screen_width,
                self.screen_height,
            ));
        }

        for chunk in self.indices.chunks(3) {
            let p1 = points_2d[chunk[0] as usize];
            let p2 = points_2d[chunk[1] as usize];
            let p3 = points_2d[chunk[2] as usize];

            if point_in_triangle(pos, p1, p2, p3) {
                return true;
            }
        }

        false
    }
}

fn point_in_triangle(p: Pos2, a: Pos2, b: Pos2, c: Pos2) -> bool {
    let area = 0.5 * (-b.y * c.x + a.y * (-b.x + c.x) + a.x * (b.y - c.y) + b.x * c.y);
    let s = 1.0 / (2.0 * area) * (a.y * c.x - a.x * c.y + (c.y - a.y) * p.x + (a.x - c.x) * p.y);
    let t = 1.0 / (2.0 * area) * (a.x * b.y - a.y * b.x + (a.y - b.y) * p.x + (b.x - a.x) * p.y);

    s >= 0.0 && t >= 0.0 && (s + t) <= 1.0
}

impl Transformable for Object {
    fn handle_keyboard(&mut self, i: &InputState) {
        let dt = i.stable_dt;

        if i.key_down(Key::F) {
            self.rotation.x += self.rotation_speed.x * dt;
        }
        if i.key_down(Key::G) {
            self.rotation.y += self.rotation_speed.y * dt;
        }
        if i.key_down(Key::H) {
            self.rotation.z += self.rotation_speed.z * dt;
        }

        let move_speed = 10.0 * dt;

        if i.key_down(Key::W) {
            self.translation.z += move_speed;
        }
        if i.key_down(Key::S) {
            self.translation.z -= move_speed;
        }
        if i.key_down(Key::A) {
            self.translation.x -= move_speed;
        }
        if i.key_down(Key::D) {
            self.translation.x += move_speed;
        }
        if i.key_down(Key::Q) {
            self.translation.y += move_speed;
        }
        if i.key_down(Key::E) {
            self.translation.y -= move_speed;
        }

        if i.key_pressed(Key::X) {
            self.mirror_x = !self.mirror_x;
        }
        if i.key_pressed(Key::Y) {
            self.mirror_y = !self.mirror_y;
        }
        if i.key_pressed(Key::Z) {
            self.mirror_z = !self.mirror_z;
        }

        let scale_speed = 10.0 * dt;
        if i.key_down(Key::P) {
            self.scale.x += scale_speed;
        }
        if i.key_down(Key::O) {
            self.scale.y += scale_speed;
        }
        if i.key_down(Key::I) {
            self.scale.z += scale_speed;
        }

        if i.key_down(Key::L) {
            self.scale.x -= scale_speed;
        }
        if i.key_down(Key::K) {
            self.scale.y -= scale_speed;
        }
        if i.key_down(Key::J) {
            self.scale.z -= scale_speed;
        }
    }
}

fn get_perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    let f = 1.0 / (fov.to_radians() / 2.0).tan();

    Matrix4::new(
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        (far + near) / (near - far),
        (2.0 * far * near) / (near - far),
        0.0,
        0.0,
        -1.0,
        0.0,
    )
}

fn world_to_screen(ndc: Vector3<f32>, screen_width: f32, screen_height: f32) -> Pos2 {
    let x = (ndc.x + 1.0) * 0.5 * screen_width;
    let y = (1.0 - ndc.y) * 0.5 * screen_height;

    Pos2::new(x, y)
}
