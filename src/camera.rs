use crate::utils::Vec3;
use nalgebra::{Matrix4, RowVector4};

pub struct Frustum {
    // 垂直视野（弧度）
    pub fov: f32,
    // 宽高比
    pub aspect: f32,
    // 近平面（距离）
    pub near: f32,
    // 远平面（距离）
    pub far: f32,
}

pub struct Camera {
    frustum: Frustum,
    position: Vec3,
    look_at: Vec3,
    up_direction: Vec3,
}

impl Camera {
    pub fn set_model_matrix(rotation_angle: f64) -> Matrix4<f64> {
        let (sin, cos) = rotation_angle.to_radians().sin_cos();
        Matrix4::from_rows(&[
            RowVector4::new(cos, -sin, 0.0, 0.0),
            RowVector4::new(sin, cos, 0.0, 0.0),
            RowVector4::new(0.0, 0.0, 1.0, 0.0),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }
    pub fn set_view_matrix(eye_pos: Vec3) -> Matrix4<f64> {
        Matrix4::from_rows(&[
            RowVector4::new(1.0, 0.0, 0.0, -eye_pos.x),
            RowVector4::new(0.0, 1.0, 0.0, -eye_pos.y),
            RowVector4::new(0.0, 0.0, 1.0, -eye_pos.z),
            RowVector4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }

    pub fn set_projection_matrix(
        eye_fov: f64,
        aspect_radio: f64,
        z_near: f64,
        z_far: f64,
    ) -> Matrix4<f64> {
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
        m2 * m1
    }
}
