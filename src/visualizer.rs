use plotters::prelude::*;


pub fn scatter_3d(points: Vec<[i8; 3]>, path: &str) {
    let canvas = SVGBackend::new(path, (1024, 760)).into_drawing_area();

    canvas.fill(&WHITE).unwrap();

    let x_axis = 0.0..10.0;
    let y_axis = 0.0..10.0;
    let z_axis = 0.0..10.0;
    // let x_axis = (-3.0..3.0).step(0.1);
    // let z_axis = (-3.0..3.0).step(0.1);

    let mut plot = ChartBuilder::on(&canvas)
        .caption("fun plotting", ("sans", 20))
        .build_cartesian_3d(x_axis, y_axis, z_axis)
        .unwrap();
    
    // chart.with_projection()

    plot
        .configure_axes()
        .draw()
        .unwrap();

    // plot.draw_series(
    //     points.iter().map(|point| Cuboid::new(*point, 5, &BLUE))
    // );

    canvas.present().expect("unable to write");
}