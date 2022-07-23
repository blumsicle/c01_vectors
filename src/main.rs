use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    location: Vec2,
    velocity: Vec2,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            location: vec2(100.0, 100.0),
            velocity: vec2(2.5, 5.0),
        }
    }
}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title("noc_c01_debug")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    Model {
        ..Default::default()
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.location += model.velocity;

    let bounds = app.window_rect();

    if model.location.x <= bounds.left() || model.location.x >= bounds.right() {
        model.velocity.x *= -1.0;
    }

    if model.location.y <= bounds.bottom() || model.location.y >= bounds.top() {
        model.velocity.y *= -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(rgb(0.2, 0.4, 0.6));

    draw.ellipse()
        .color(rgb8(175, 175, 175))
        .radius(16.0)
        .xy(model.location);

    draw.to_frame(app, &frame).unwrap();
}
