use crate::utils::Vec3;
use fltk::{app::set_visual, enums::Mode, prelude::*, window::Window};

pub fn create_window(w: u32, h: u32, frame_buff: Vec<Vec3>) {
    let app = fltk::app::App::default();
    let mut wind = Window::new(100, 100, w as i32, h as i32, "Hello from rust");

    let mut v8: Vec<u8> = Vec::new();
    frame_buff.into_iter().for_each(|v| {
        v8.push(v.x as u8);
        v8.push(v.y as u8);
        v8.push(v.z as u8);
    });
    fltk::draw::draw_image(&v8, 0, 0, w as i32, h as i32, fltk::enums::ColorDepth::Rgb8);
    wind.end();
    set_visual(Mode::Rgb).unwrap();
    wind.show();
    app.run().unwrap();
}
