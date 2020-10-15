use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    //Clear the background to purple.
    draw.background().color(PLUM);

    // ellipse
    draw.ellipse()
        .color(STEELBLUE)
        .w(300.0)
        .h(22.0)
        .x_y(200.0, -100.0);

    // rect
    draw.rect()
        .color(rgba(0.1, 0.1, 0.1, 0.5))
        .w(300.0)
        .h(200.0);

    // quad
    let p1 = pt2(-10.0, -20.0);
    let p2 = pt2(10.0, -40.0);
    let p3 = pt2(15.0, 40.0);
    let p4 = pt2(-20.0, 35.0);

    draw.quad()
        .color(YELLOW)
        .w(125.0)
        .h(77.0)
        .points(p1, p2, p3, p4);

    // tri
    draw.tri()
        .color(BROWN)
        .w(125.0)
        .h(77.0)
        .x_y(-200.0, 300.0)
        .points(p1, p2, p3);

    // line
    let start_point = pt2(-30.0, -20.0);
    let end_point = pt2(40.0, 40.0);

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(40.0)
        .color(STEELBLUE);

    // polyline
    let poly_points = (0..5000).map(|i| {
        let x = i as f32 / 10.0 - 100.0;
        let point = pt2(x, x.sin()) * 20.0;
        (point, STEELBLUE)
    });

    draw.polyline().weight(3.0).points_colored(poly_points);

    // Write to the window frame
    draw.to_frame(app, &frame).unwrap();
}
