use tiny_skia::*;

fn main() {
    let mut pixmap = Pixmap::new(800, 500).unwrap();

    //draw_polygon1(&mut pixmap);
    //draw_polygon2(&mut pixmap);
    //draw_polygon3(&mut pixmap);
    //draw_polygon4(&mut pixmap);

    pixmap.save_png("out.png").unwrap();
}

fn draw_filled_polygon(pixmap: &mut Pixmap, points: &[(f32, f32)], fill: Color, stroke_color: Color) {
    let mut pb = PathBuilder::new();
    pb.move_to(points[0].0, points[0].1);
    for &(x, y) in &points[1..] {
        pb.line_to(x, y);
    }
    pb.close();
    let path = pb.finish().unwrap();

    
    let mut paint = Paint::default();
    paint.shader = Shader::SolidColor(fill); 
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    
    let mut stroke_paint = Paint::default();
    stroke_paint.shader = Shader::SolidColor(stroke_color); 
    let mut stroke = Stroke::default();
    stroke.width = 1.5;
    pixmap.stroke_path(
        &path,
        &stroke_paint,
        &stroke,
        Transform::identity(),
        None,
    );
}

fn draw_polygon1(pixmap: &mut Pixmap) {
    let points = [
        (165.0, 380.0), (185.0, 360.0), (180.0, 330.0), (207.0, 345.0),
        (233.0, 330.0), (230.0, 360.0), (250.0, 380.0), (220.0, 385.0),
        (205.0, 410.0), (193.0, 383.0),
    ];
    draw_filled_polygon(pixmap, &points, Color::from_rgba8(255, 255, 0, 255), Color::WHITE);
}

fn draw_polygon2(pixmap: &mut Pixmap) {
    let points = [
        (321.0, 335.0), (288.0, 286.0), (339.0, 251.0), (374.0, 302.0),
    ];
    draw_filled_polygon(pixmap, &points, Color::from_rgba8(0, 0, 255, 255), Color::WHITE);
}

fn draw_polygon3(pixmap: &mut Pixmap) {
    let points = [
        (377.0, 249.0), (411.0, 197.0), (436.0, 249.0),
    ];
    draw_filled_polygon(pixmap, &points, Color::from_rgba8(255, 0, 0, 255), Color::WHITE);
}

fn draw_polygon4(pixmap: &mut Pixmap) {
    let outer = vec![
        (413.0, 177.0), (448.0, 159.0), (502.0, 88.0), (553.0, 53.0), (535.0, 36.0),
        (676.0, 37.0), (660.0, 52.0), (750.0, 145.0), (761.0, 179.0), (672.0, 192.0),
        (659.0, 214.0), (615.0, 214.0), (632.0, 230.0), (580.0, 230.0), (597.0, 215.0),
        (552.0, 214.0), (517.0, 144.0), (466.0, 180.0),
    ];
    let hole = vec![
        (739.0, 170.0), (735.0, 148.0), (708.0, 120.0), (682.0, 175.0),
    ]; 

    let mut pb = PathBuilder::new();

    
    pb.move_to(outer[0].0, outer[0].1);
    for &(x, y) in &outer[1..] {
        pb.line_to(x, y);
    }
    pb.close();

    
    pb.move_to(hole[0].0, hole[0].1);
    for &(x, y) in &hole[1..] {
        pb.line_to(x, y);
    }
    pb.close();

    let path = pb.finish().unwrap();

    
    let mut paint = Paint::default();
    paint.shader = Shader::SolidColor(Color::from_rgba8(0, 255, 0, 255)); 
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::EvenOdd,
        Transform::identity(),
        None,
    );

    
    let mut stroke_paint = Paint::default();
    stroke_paint.shader = Shader::SolidColor(Color::WHITE);
    let mut stroke = Stroke::default();
    stroke.width = 1.5;
    pixmap.stroke_path(
        &path,
        &stroke_paint,
        &stroke,
        Transform::identity(),
        None,
    );
}
