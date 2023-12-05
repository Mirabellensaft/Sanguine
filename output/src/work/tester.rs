use rand::thread_rng;
use rand::Rng;
use sanguine_lib::resources::border_coordinates::AllBorderCoordinates;
use sanguine_lib::resources::composition::grid::CompositionOverlay;
use sanguine_lib::resources::exclusion::Exclusion;
use sanguine_lib::resources::{
    layout,
    shapes::{circle::Circle, line::Line, point::Point, Shape},
};
use svg::node;
use svg::Node;

pub fn form_group(layout: &layout::Grid, ex: &Exclusion) -> node::element::Group {
    let mut graph = node::element::Group::new();

    let mut comp = CompositionOverlay::new_flat(layout);

    let mut all_coords = AllBorderCoordinates::new(layout, 10);
    all_coords.tesselate();
    all_coords.slight_chaos();

    for row in 0..layout.rows {
        for col in 0..layout.columns {
            let mut rng = thread_rng();

            let radius = rng.gen_range(7..=15);

            let center = Point::random_coordinate(
                &layout.field_container[row as usize][col as usize],
                radius * 2,
            );

            if ex.0[0].contains(center) == false {
                let circle = Circle::new(center, radius as f32);
                graph.append(circle.draw());

                for side in 0..4 {
                    let the_side = &all_coords.0[row][col].0[side];
                    let circle = Circle::new(center, radius as f32);

                    for point in 0..10 {
                        if ex.0[0].contains(all_coords.0[row][col].0[side].0[point]) {
                            let prelim_line = Line::new(
                                all_coords.0[row][col].0[side].0[point],
                                ex.0[0].return_center(),
                            );
                            let step = 1.0;

                            if let Some(endpoint) = ex.0[0].intersection(prelim_line, step) {
                                // println!("Point {:?}, EP {:?}", point, endpoint);
                                let line =
                                    Line::new(all_coords.0[row][col].0[side].0[point], endpoint);
                                graph.append(line.draw());
                            } else {
                                // println!("shit!");
                            };
                        } else {
                            let prelim_line =
                                Line::new(all_coords.0[row][col].0[side].0[point], circle.center);
                            let step = 1.0;

                            if let Some(endpoint) = circle.intersection(prelim_line, step) {
                                // println!("Point {:?}, EP {:?}", point, endpoint);
                                let line =
                                    Line::new(all_coords.0[row][col].0[side].0[point], endpoint);
                                graph.append(line.draw());
                            } else {
                                // println!("shit!");
                            };
                        }
                    }
                }
            } else {
                for side in 0..4 {
                    // let the_side = &all_coords.0[row][col].0[side];
                    // let circle = shapes::circle::Circle::new(center, radius as f32);

                    for point in 0..10 {
                        if ex.0[0].contains(all_coords.0[row][col].0[side].0[point]) == false {
                            let prelim_line = Line::new(
                                all_coords.0[row][col].0[side].0[point],
                                ex.0[0].return_center(),
                            );
                            let step = 1.0;

                            if let Some(endpoint) = ex.0[0].intersection(prelim_line, step) {
                                // println!("Point {:?}, EP {:?}", point, endpoint);
                                let line =
                                    Line::new(all_coords.0[row][col].0[side].0[point], endpoint);
                                graph.append(line.draw());
                            } else {
                                // println!("shit!");
                            };
                        }
                    }
                }
            }
        }
    }

    graph
}
