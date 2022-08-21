use nannou::{prelude::*};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let dims = app.window_rect().w_h();
    let end  = dims.0 as i32 / 2;
    let start =  -end;

    for i in start..end {
        let x = i as f32 + app.time / 4.0;
        draw.ellipse().radius(1.0).stroke_weight(1.0).color(PURPLE).x_y(x, x.sin() * dims.1/2.0);
        draw.ellipse().radius(1.0).stroke_weight(1.0).color(BLUE).x_y(x, x.sin() * dims.1/2.5);
        draw.ellipse().radius(1.0).stroke_weight(1.0).color(GREEN).x_y(x, x.sin() * dims.1/3.5);
    }

    draw.to_frame(app, &frame).unwrap();
}