use std::collections::HashMap;

use crate::utils::{Rgb, Triangle};
use nalgebra::{Matrix4, Vector3, Vector4};

type Vec3 = Vector3<f64>;
type Vec4 = Vector4<f64>;

pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

#[derive(Default)]
pub struct Rasterizer {
    width: u32,
    height: u32,
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    pos_buf: HashMap<usize, Vec<Vec3>>,
    depth_buf: Vec<f64>,
    next_id: usize,
    pub frame_buf: Vec<Vec3>,
}

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Rasterizer {
        let mut r = Rasterizer::default();
        r.width = width;
        r.height = height;
        r.frame_buf
            .resize((width * height) as usize, Vector3::zeros());
        r.depth_buf.resize((width * height) as usize, 0.0);
        r
    }
    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }
    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }
    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    pub fn set_pixel(
        width: u32,
        height: u32,
        point: Vec3,
        color: &Vec3,
        frame_buf: &mut Vec<Vec3>,
    ) {
        if point.x < 0.0 || point.x > width as f64 || point.y < 0.0 || point.y > height as f64 {
            return;
        }
        let ind = (height as f64 - 1.0 - point.y) * width as f64 + point.x;
        frame_buf[ind as usize] = *color;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }
    pub fn load_position(&mut self, positions: &Vec<Vec3>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn clear(&mut self) {}
    // 使用 Bresenham's line algorithm 算法绘制直线, 其过程本质是已知两点的直线连续函数，求其离散化的最佳表示
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn draw_line(width: u32, height: u32, begin: &Vec3, end: &Vec3, frame_buf: &mut Vec<Vec3>) {
        let line_color = Vector3::new(255.0, 255.0, 255.0);
        let (x1, y1) = (begin.x, begin.y);
        let (x2, y2) = (end.x, end.y);
        let (dx, dy) = (x2 - x1, y2 - y1);
        let (dx1, dy1) = (dx.abs(), dy.abs());

        let mut px = 2.0 * dy1 - dx1;
        let mut py = 2.0 * dx1 - dy1;
        if dy <= dx {
            // 起始点反向画
            let (mut x, mut y, xe) = if dx >= 0.0 {
                (x1, y1, x2)
            } else {
                (x2, y2, x1)
            };
            let point = Vec3::new(x1.round(), x2.round(), 1.0);
            Self::set_pixel(width, height, point, &line_color, frame_buf);
            while x < xe {
                x += 1.0;
                if px < 0.0 {
                    px += 2.0 * dy1;
                } else {
                    // 算法拓展到负数斜率
                    if (dy < 0.0 && dx < 0.0) || (dx > 0.0 && dy > 0.0) {
                        y += 1.0;
                    } else {
                        y -= 1.0;
                    }
                    px += 2.0 * (dy1 - dx1);
                }
                let p = Vec3::new(x.round(), y.round(), 1.0);
                Self::set_pixel(width, height, p, &line_color, frame_buf);
            }
        } else {
            let (mut x, mut y, ye) = if dy >= 0.0 {
                (x1, y1, y2)
            } else {
                (x2, y2, y1)
            };
            let point = Vec3::new(x1.round(), x2.round(), 1.0);
            Self::set_pixel(width, height, point, &line_color, frame_buf);
            while y < ye {
                y += 1.0;
                if py < 0.0 {
                    py += 2.0 * dx1;
                } else {
                    // 算法拓展到负数斜率
                    if (dy < 0.0 && dx < 0.0) || (dx > 0.0 && dy > 0.0) {
                        x += 1.0;
                    } else {
                        x -= 1.0;
                    }
                    py += 2.0 * (dx1 - dy1);
                }
                let p = Vec3::new(x.round(), y.round(), 1.0);
                Self::set_pixel(width, height, p, &line_color, frame_buf);
            }
        }
        // 开始点
    }

    pub fn draw(&mut self, pos_id: PosBufId, ind_id: IndBufId, _type: Primitive) {
        let buf = &self.pos_buf[&pos_id.0];
        let ind = &self.ind_buf[&ind_id.0];
        let mvp = self.projection * self.view * self.model;
        let frame_buf = &mut self.frame_buf;
        for i in ind {
            let t = Rasterizer::get_triangle(self.width, self.height, buf, mvp, i);
            Self::draw_line(self.width, self.height, &t.a(), &t.b(), frame_buf);
            Self::draw_line(self.width, self.height, &t.b(), &t.c(), frame_buf);
            Self::draw_line(self.width, self.height, &t.c(), &t.a(), frame_buf);
        }
    }

    fn get_triangle(
        width: u32,
        height: u32,
        buf: &Vec<Vec3>,
        mvp: Matrix4<f64>,
        i: &Vector3<usize>,
    ) -> Triangle {
        let zfar = 50.0;
        let znear = 0.1;
        let f1 = (zfar - znear) / 2.0;
        let f2 = (zfar + znear) / 2.0;

        let mut t = Triangle::new();
        let mut v = vec![
            mvp * to_vec4(buf[i[0]], Some(1.0)),
            mvp * to_vec4(buf[i[1]], Some(1.0)),
            mvp * to_vec4(buf[i[2]], Some(1.0)),
        ];

        for vec in v.iter_mut() {
            *vec = *vec / vec.w;
        }
        // 屏幕应映射，从裁剪空间到屏幕空间
        // [-1, 1] -> [0, 700]
        for vert in v.iter_mut() {
            vert.x = 0.5 * width as f64 * (vert.x + 1.0);
            vert.y = 0.5 * height as f64 * (vert.y + 1.0);
            vert.z = vert.z * f1 + f2;
        }
        for j in 0..3 {
            t.set_vertex(j, v[j].xyz());
        }

        t.set_color(0, Rgb(255, 0, 0));
        t.set_color(1, Rgb(0, 255, 0));
        t.set_color(2, Rgb(0, 0, 255));
        t
    }
    // 判断点是否在三角形内部
    pub fn inside_triangle(x: f64, y: f64, triangle: &Triangle) -> bool {
        let p = Vec3::new(x, y, 0.0);
        let ap = p - triangle.a();
        let bp = p - triangle.b();
        let cp = p - triangle.c();

        let ab = triangle.b() - triangle.a();
        let bc = triangle.c() - triangle.b();
        let ca = triangle.a() - triangle.c();

        let i = ap.cross(&ab);
        let j = bp.cross(&bc);
        let k = cp.cross(&ca);
        (i.z > 0.0 && j.z > 0.0 && k.z > 0.0) || (i.z < 0.0 && j.z < 0.0 && k.z < 0.0)
    }

    pub fn rasterize_triangle(triangle: &Triangle) {
        // 创建 bounding box
        let min_x = triangle.a().x.min(triangle.b().x).min(triangle.c().x) as usize;
        let min_y = triangle.a().y.min(triangle.b().y).min(triangle.c().y) as usize;
        let max_x = triangle.a().x.max(triangle.b().x).max(triangle.c().x) as usize;
        let max_y = triangle.a().y.max(triangle.b().y).max(triangle.c().y) as usize;
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !Self::inside_triangle(x as f64 + 0.5, y as f64 + 0.5, triangle) {
                    continue;
                }
            }
        }
    }
}

fn to_vec4(v3: Vec3, w: Option<f64>) -> Vec4 {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}
