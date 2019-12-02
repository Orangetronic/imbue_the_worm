use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    positions: Vec<Vector2>
}

fn model(_app: &App) -> Model {
    Model {
        positions: Vec::new()
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    let win  = app.window_rect();

    let p = pt2(
        win.x() + app.mouse.x,
        win.y() + app.mouse.y
    );

    model.positions.insert(0, p);
    model.positions.truncate(100);

}



fn view(app: &App, model: &Model, frame: &Frame){

    frame.clear(PURPLE);

    let draw = app.draw();

    let length = model.positions.len();

    for i in 0..length {

        let p = model.positions[i];

        let alpha = 100.0 - i as f32;
        let size  = 120.0 - i as f32;
        let hue   = 0.3 - (i as f32 / 100.0);

        draw
        .ellipse()
        .width(size)
        .height(size)
        .xy(p)
        .hsla(hue, 0.8, 0.8, alpha / 100.0);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

}
