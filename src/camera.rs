use crate::utils::Vec3;
use nalgebra::{Matrix4, Rotation, RowVector4};

struct Frustum {
    // 垂直视野（弧度）
    pub fov: f64,
    // 宽高比
    pub aspect: f64,
    // 近平面（距离）
    pub near: f64,
    // 远平面（距离）
    pub far: f64,
}

pub struct Camera {
    frustum: Frustum,
    position: Vec3,
    look_at: Vec3,
    up_direction: Vec3,

    pub view_matrix: Matrix4<f64>,
    pub projection_matrix: Matrix4<f64>,
}

impl Camera {
    pub fn new(position: Vec3, fov: f64, aspect: f64, near: f64, far: f64) -> Self {
        let frustum = Frustum {
            fov,
            aspect,
            near,
            far,
        };
        let mut camera = Camera {
            frustum,
            position,
            look_at: Vec3::new(0.0, 0.0, -1.0),
            up_direction: Vec3::new(0.0, 1.0, 0.0),
            view_matrix: Matrix4::default(),
            projection_matrix: Matrix4::default(),
        };
        camera.set_view_matrix();
        camera.set_projection_matrix();
        camera
    }

    fn set_view_matrix(&mut self) {
        let eye_pos = self.position;
        //  平移
        let transform = Matrix4::from_rows(&[
            RowVector4::new(1.0, 0.0, 0.0, -eye_pos.x),
            RowVector4::new(0.0, 1.0, 0.0, -eye_pos.y),
            RowVector4::new(0.0, 0.0, 1.0, -eye_pos.z),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ]);
        let p = self.look_at.cross(&self.up_direction);
        let rotation = Matrix4::from_rows(&[
            RowVector4::new(p.x, p.y, p.z, 0.0),
            RowVector4::new(
                self.up_direction.x,
                self.up_direction.y,
                self.up_direction.z,
                0.0,
            ),
            RowVector4::new(-self.look_at.x, -self.look_at.y, -self.look_at.z, 0.0),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ])
        .transpose();
        self.view_matrix = transform * rotation;
    }

    fn set_projection_matrix(&mut self) {
        let frustum = &self.frustum;
        let [z_near, z_far, eye_fov, aspect_radio] =
            [frustum.near, frustum.far, frustum.fov, frustum.aspect];

        // 透视矩阵
        let m1 = Matrix4::from_rows(&[
            RowVector4::new(z_near, 0.0, 0.0, 0.0),
            RowVector4::new(0.0, z_near, 0.0, 0.0),
            RowVector4::new(0.0, 0.0, z_near + z_far, -z_near * z_far),
            RowVector4::new(0.0, 0.0, 1.0, 0.0),
        ]);

        // 正交矩阵
        let t = -z_near.abs() * (eye_fov / 2.0).to_radians().tan();
        let r = aspect_radio * t;
        let l = -r;
        let b = -t;
        let m2 = Matrix4::from_rows(&[
            RowVector4::new(2.0 / (r - l), 0.0, 0.0, 0.0),
            RowVector4::new(0.0, 2.0 / (t - b), 0.0, 0.0),
            RowVector4::new(
                0.0,
                0.0,
                2.0 / (z_near - z_far),
                -(z_near + z_far) / (z_near - z_far),
            ),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ]);
        self.projection_matrix = m2 * m1;
    }
}
