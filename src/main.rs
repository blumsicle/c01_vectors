use std::ops::{Mul, Sub};

use nannou::prelude::*;

struct Mover {
    location: Vec2,
    velocity: Vec2,
    color: Hsv,
}

impl Mover {
    fn update(&mut self) {
        self.location += self.velocity;
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .color(self.color)
            .radius(8.0)
            .xy(self.location);
    }

    fn check_edges(&mut self, bounds: &Rect) {
        if self.location.x < bounds.left() {
            self.location.x = bounds.right();
        } else if self.location.x > bounds.right() {
            self.location.x = bounds.left();
        }

        if self.location.y < bounds.bottom() {
            self.location.y = bounds.top();
        } else if self.location.y > bounds.top() {
            self.location.y = bounds.bottom();
        }
    }
}

struct Model {
    movers: Vec<Mover>,
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title("noc_c01_debug")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let bounds = app.window_rect();
    let mut movers = Vec::<Mover>::new();

    for _ in 0..10 {
        let location = random::<Vec2>().sub(0.5).mul(bounds.wh());
        let velocity = random::<Vec2>().sub(0.5).mul(10.0);
        let color = hsv(random_f32(), 0.5, 0.7);

        movers.push(Mover {
            location,
            velocity,
            color,
        })
    }

    Model { movers }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();

    for mover in &mut model.movers {
        mover.update();
        mover.check_edges(&bounds);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(rgb(0.2, 0.4, 0.6));

    for mover in &model.movers {
        mover.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
