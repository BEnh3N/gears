use std::f32::consts::PI;
use std::time::Instant;
use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Polygon;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const W2: f32 = WIDTH as f32 / 2.;
const H2: f32 = HEIGHT as f32 / 2.;

fn to_screen_space<V: Into<Vec2>>(vec: V) -> Vec2 {
    let v = vec.into();
    let x= v.x + W2;
    let y = -v.y + H2;
    Vec2 { x, y }
}

fn rotate<V: Into<Vec2>>(vec: V, theta: f32) -> Vec2 {
    let v = vec.into();
    let x = v.x * theta.cos() - v.y * theta.sin();
    let y = v.x * theta.sin() + v.y * theta.cos();
    Vec2 { x, y }
}

fn translate<V: Into<Vec2>>(vec: V, radius: f32) -> Vec2 {
    let v = vec.into();
    Vec2 { x: v.x + radius, y: v.y}
}

fn input_gear_fn(s: f32) -> Vec2 {
    let r = 10. * (s*20.*PI).sin() + 100.;
    Vec2 {
        x: r * (s * 2.*PI).cos(),
        y: r * (s * 2.*PI).sin()
    }
}

struct Gear {
    f: fn(f32) -> Vec2,
    axle_separation: f32,
    source_w: f32,
    partner_w: f32,
}

impl Gear {
    fn draw(&self, t: f32, segments: u32, graphics: &mut Graphics2D) {
        let rotation = self.source_w * t;
        let revolution = self.partner_w * t;
        let segment_length = 1. / segments as f32;

        let mut vertices = vec![];
        for i in 0..segments {
            let p = (self.f)(i as f32 * segment_length);
            let rotated_p = rotate(p, rotation);
            let translated_p = translate(rotated_p, self.axle_separation);
            let rotated_p2 = rotate(translated_p, revolution);
            vertices.push(to_screen_space(rotated_p2));
        }
        let shape = Polygon::new(&vertices);
        graphics.draw_polygon(&shape, (0., 0.), Color::WHITE)
    }
}

struct MyWindowHandler{
    start_t: Instant,
    t: f32,
    gear: Gear
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);
        self.gear.draw(self.t, 100, graphics);

        self.t = self.start_t.elapsed().as_secs_f32();
        helper.request_redraw();
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        let key = virtual_key_code.unwrap();
        match key {
            VirtualKeyCode::Escape => helper.terminate_loop(),
            _ => {}
        }
    }
}

fn main() {
    let window = Window::new_centered("Gears", (WIDTH, HEIGHT)).expect("Failed to create window!");
    let start_t = Instant::now();
    window.run_loop(MyWindowHandler{
        start_t,
        t: start_t.elapsed().as_secs_f32(),
        gear: Gear {
            f: input_gear_fn,
            axle_separation: 250.,
            source_w: PI/2.,
            partner_w: PI/4.,
        }
    });
}
