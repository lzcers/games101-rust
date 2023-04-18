mod rgb;
mod traingle;
mod transform;

pub use rgb::Rgb;
pub use traingle::Triangle;
pub use transform::*;

use nalgebra::Vector3;
use opencv::core::{Mat, MatTraitConst};
use opencv::imgproc::{cvt_color, COLOR_RGB2BGR};
use std::os::raw::c_void;

pub type Vec3 = Vector3<f64>;

pub fn frame_buffer2cv_mat(frame_buffer: &Vec<Vec3>) -> opencv::core::Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700,
            700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        )
        .unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image
        .convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0)
        .expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    image
}
