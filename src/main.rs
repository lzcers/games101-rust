mod display;
mod rasterizer;
mod utils;

use nalgebra::Vector3;
use opencv::highgui::{imshow, wait_key};
use rasterizer::{Primitive, Rasterizer};
use utils::{
    frame_buffer2cv_mat, get_model_matrix, get_projection_matrix, get_view_matrix, Triangle,
};
type Vec3 = nalgebra::Vector3<f64>;

fn main() {
    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![
        Vector3::new(2.0, 0.0, -2.0),
        Vector3::new(0.0, 2.0, -2.0),
        Vector3::new(-2.0, 0.0, -2.0),
    ];
    let ind = vec![Vector3::new(0, 1, 2)];

    let pos_id = r.load_position(&pos);
    let ind_id = r.load_indices(&ind);

    let mut angle = 0.0;
    let mut k = 0;
    while k != 27 {
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(50.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, Primitive::Triangle);
        let image = frame_buffer2cv_mat(&r.frame_buf);

        imshow("image", &image).unwrap();

        k = wait_key(80).unwrap();
        if k == 'a' as i32 {
            angle += 10.0;
        } else if k == 'd' as i32 {
            angle -= 10.0;
        }
    }
}
