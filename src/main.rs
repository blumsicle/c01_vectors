use nannou::prelude::*;

mod mover;
use mover::Mover;

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
        let location = (random::<Vec2>() - 0.5) * bounds.wh();
        let color = hsv(random_f32(), 0.5, 0.7);

        movers.push(Mover::new(location, color));
    }

    Model { movers }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let bounds = app.window_rect();
    let mouse = vec2(app.mouse.x, app.mouse.y);

    for mover in &mut model.movers {
        mover.update(&mouse);
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
