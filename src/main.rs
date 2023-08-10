mod display;
mod rasterizer;
mod utils;

use display::create_window;
use nalgebra::Vector3;
use rasterizer::{Primitive, Rasterizer};
use utils::{get_model_matrix, get_projection_matrix, get_view_matrix};

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
    r.set_model(get_model_matrix(angle));
    r.set_view(get_view_matrix(eye_pos));
    r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
    r.draw(pos_id, ind_id, Primitive::Triangle);
    create_window(700, 700, r.frame_buf);
}
