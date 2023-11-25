use svg::node;
use svg::Node;

use rand::{thread_rng, Rng};

use sanguine_lib::resources::{layout, border_coordinates, shapes};

pub fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();

    let mut all_coords = border_coordinates::AllBorderCoordinates::new(layout, 10);
    all_coords.tesselate(layout);

    // let coordinates = shapes::Point::random_coordinates_on_border(&layout.field_container[row as usize][col as usize]);

    for row in 0..layout.rows {
        for col in 0..layout.rows {
            let mut rng = thread_rng();
            let radius = rng.gen_range(3..=10);

            let center = shapes::Point::random_coordinate(
                &layout.field_container[row as usize][col as usize],
                radius * 2,
            );

            let circle = shapes::Circle::new(center, radius as f32);
            graph.append(circle.draw());

            // println!(
            //     "COORDS {:?}",
            //     all_coords[row][col]
            // );

            for side in 0..4 {
                for point in 0..10 {
                    print!(
                        "{}, {}, start: {:?}, center:{:?} \n",
                        side, point, all_coords.0[row][col].0[side].0[point], center
                    );
                    let prelim_line = shapes::Line::new(all_coords.0[row][col].0[side].0[point], center);
                    let step = 1.0;

                    if let Some(endpoint) = circle.intersection(prelim_line, step) {
                        println!("Point {:?}, EP {:?}", point, endpoint);
                        let line = shapes::Line::new(all_coords.0[row][col].0[side].0[point], endpoint);
                        graph.append(line.draw());
                    };
                }
            }
        }
    }

    graph
}
