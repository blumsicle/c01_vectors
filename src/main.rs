use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    let _ = app
        .new_window()
        .title("noc_c01_debug")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(rgb(0.2, 0.4, 0.6));

    draw.to_frame(app, &frame).unwrap();
}
