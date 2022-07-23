use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

impl Default for Model {
    fn default() -> Self {
        Self {}
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

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let mut mouse = vec2(app.mouse.x, app.mouse.y);
    mouse *= 0.5;

    draw.background().color(rgb(0.2, 0.4, 0.6));

    draw.line()
        .color(rgb(0.2, 0.6, 0.4))
        .points(Vec2::ZERO, mouse)
        .weight(2.0);

    draw.ellipse()
        .color(rgb(0.4, 0.8, 0.6))
        .radius(8.0)
        .xy(mouse);

    draw.to_frame(app, &frame).unwrap();
}
