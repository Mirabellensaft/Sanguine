use svg::node;
use svg::Node;

use rand::{thread_rng, Rng};

use sanguine_lib::resources::{
    layout::{self, Orientation},
    random_numbers,
    shapes::{self, Point},
};

pub fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();

    let mut all_coords = random_numbers::all_border_coordinates(layout);
    let mut counter = 0;

    for row in 0..layout.rows {
        for col in 0..layout.columns {
            if row == 0 && col != 0 {
                all_coords[row][col][1] = all_coords[row][col - 1][3];
            } else if row != 0 && col == 0 {
                all_coords[row][col][0] = all_coords[row - 1][col][2];
            } else if row != 0 && col != 0 {
                all_coords[row][col][1] = all_coords[row][col - 1][3];
                all_coords[row][col][0] = all_coords[row - 1][col][2];
            }
        }
    }

    // let coordinates = random_numbers::coordinates_on_border(&layout.field_container[row as usize][col as usize]);

    for row in 0..layout.rows {
        for col in 0..layout.rows {
            let mut rng = thread_rng();
            let radius = rng.gen_range(3..=10);

            let center = random_numbers::coordinate(
                &layout.field_container[row as usize][col as usize],
                radius,
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
                        side, point, all_coords[row][col][side][point], center
                    );
                    let mut prelim_line =
                        shapes::Line::new(all_coords[row][col][side][point], center);
                    let step = 1.0;

                    if let Some(endpoint) = circle.intersection(prelim_line, step) {
                        println!("Point {:?}, EP {:?}", point, endpoint);
                        let line = shapes::Line::new(all_coords[row][col][side][point], endpoint);
                        graph.append(line.draw());
                    };
                }
            }
        }
    }

    graph
}
