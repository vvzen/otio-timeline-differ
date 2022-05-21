use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update){

}

fn view(app: &App, _model: &Model, frame: Frame){

    let canvas = app.draw();
    let window = app.window_rect();
    let width = window.w() as i32;
    let height = window.h() as f32;
    let step_size = 10;
    
    let start = -width / 2;
    let end = width / 2;
    let mut num_lines = 0;

    for i in (start..end).step_by(step_size){

        let current_x = i as f32;

        let show_thicker_line = match num_lines % 10 {
            0 => true,
            _ => false
        };

        let current_weight = if show_thicker_line {2.0} else {0.5} as f32;
        let color = if show_thicker_line {BLACK} else {PURPLE};       

        canvas.line()
            .start(pt2(current_x, -height))
            .end(pt2(current_x, height))
            .weight(current_weight)
            .color(color);

        num_lines = num_lines + 1;
    }

    canvas.background().color(WHITE);

    canvas.to_frame(app, &frame).unwrap();
}