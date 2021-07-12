use std::{fs::File, io::Write};

use num::ToPrimitive;

use crate::{
    canvas::Canvas,
    color::Color,
    matrix::{Axis, Matrix},
    tuple::{IsTuple, Point},
};

pub fn make_clock() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let canvas_height = 600;
    let canvas_width = 600;
    let mut canvas = Canvas::new(canvas_width, canvas_height);

    // let center = (Matrix::identity(4, 1.0)
    //     .translate(
    //         (canvas_width / 2).to_f64().unwrap(),
    //         (canvas_height / 2).to_f64().unwrap(),
    //         0.,
    //     )
    //     .unwrap()
    //     * origin)
    //     .unwrap();

    let num_points = 12; // Number of hours
    let mut points: Vec<Point> = Vec::with_capacity(num_points);
    let mut prev_point = Point::new(0.0, 1.0, 0.0);
    points.push(prev_point);

    let rotate = Matrix::identity(4, 1.0)
        .rotate(Axis::Z, std::f64::consts::FRAC_PI_6)
        .unwrap();
    for _ in 1..num_points {
        prev_point = (rotate.clone() * prev_point).unwrap();
        points.push(prev_point);
    }

    let color = Color::new(1.0, 0.0, 0.0);
    let translate = (Matrix::identity(4, 1.0).translate(
        (canvas_width / 2).to_f64().unwrap(),
        (canvas_height / 2).to_f64().unwrap(),
        0.,
    ))
    .unwrap();
    let center = (translate.clone() * origin).unwrap();
    canvas.write_pixel(
        center.0.x.round().to_usize().unwrap(),
        center.0.y.round().to_usize().unwrap(),
        color,
    );

    let scale = (Matrix::identity(4, 1.0).scale(
        (canvas_width / 3).to_f64().unwrap(),
        (canvas_height / 3).to_f64().unwrap(),
        1.0,
    ))
    .unwrap();
    for p in points {
        let point = ((translate.clone() * scale.clone()).unwrap() * p).unwrap();
        canvas.write_pixel(
            point.0.x.to_usize().unwrap(),
            point.0.y.to_usize().unwrap(),
            color,
        );
    }

    let mut file = File::create("clock.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_bytes()).unwrap();
}
