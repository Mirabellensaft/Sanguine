use rand::thread_rng;
use rand::Rng;
use sanguine_lib::resources::{layout, random_numbers, shapes};
use svg::node;
use svg::Node;

pub fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();
    for row in 0..layout.rows {
        for col in 0..layout.columns {
            let mut rng = thread_rng();
            let radius = rng.gen_range(3..=10);

            let center = random_numbers::coordinate(
                &layout.field_container[row as usize][col as usize],
                radius * 2,
            );

            let circle = shapes::Circle::new(center, radius as f32);
            graph.append(circle.draw());

            let point_1 = shapes::Point::new(
                circle.center.x + 3.0,
                layout.field_container[row as usize][col as usize].y as f32,
            );

            let y =
                random_numbers::coordinate(&layout.field_container[row as usize][col as usize], 1)
                    .y;
            let point_2 = shapes::Point::new(
                layout.field_container[row as usize][col as usize].x as f32,
                y,
            );

            let x =
                random_numbers::coordinate(&layout.field_container[row as usize][col as usize], 1)
                    .x;
            let point_3 = shapes::Point::new(
                x,
                layout.field_container[row as usize][col as usize].y as f32
                    + layout.field_container[row as usize][col as usize].row_height as f32,
            );

            let y =
                random_numbers::coordinate(&layout.field_container[row as usize][col as usize], 1)
                    .y;
            let point_4 = shapes::Point::new(
                layout.field_container[row as usize][col as usize].x as f32
                    + layout.field_container[row as usize][col as usize].column_width as f32,
                y,
            );

            let points = [point_1, point_2, point_3, point_4];

            for point in points {
                let step = 1.0;
                let prelim_line = shapes::Line::new(point, circle.center);
                if let Some(endpoint) = circle.intersection(prelim_line, step) {
                    println!("Point {:?}, EP {:?}", point, endpoint);
                    let line = shapes::Line::new(point, endpoint);
                    graph.append(line.draw());
                }
            }
        }
    }

    graph
}
