use crate::utils::Vec3;
use nalgebra::Matrix4;

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
    ppsition: Vec3,
}

impl Camera {
    pub fn set_model_matrix() -> Matrix4<f64> {}
}
