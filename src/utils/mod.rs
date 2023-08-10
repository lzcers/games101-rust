mod rgb;
mod traingle;
mod transform;

pub use rgb::Rgb;
pub use traingle::Triangle;
pub use transform::*;

use nalgebra::Vector3;
pub type Vec3 = Vector3<f64>;
