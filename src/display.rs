use crate::utils::Vec3;
use fltk::{app::set_visual, enums::Mode, prelude::*, window::Window};

pub fn create_window(w: u32, h: u32, frame_buff: Vec<Vec3>) {
    let app = fltk::app::App::default();
    let mut wind = Window::new(100, 100, w as i32, h as i32, "Hello from rust");

    wind.draw(move |_| {
        let v8 = frame_buff.iter().fold(Vec::<u8>::new(), |mut acc, cur| {
            acc.append(&mut vec![cur.x as u8, cur.y as u8, cur.z as u8]);
            acc
        });
        fltk::draw::draw_image(&v8, 0, 0, w as i32, h as i32, fltk::enums::ColorDepth::Rgb8)
            .unwrap();
    });
    set_visual(Mode::Rgb).unwrap();
    wind.end();
    wind.show();

    app.run().unwrap();
}
