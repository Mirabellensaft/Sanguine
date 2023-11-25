use svg::node;
use svg::Node;

use rand::{thread_rng, Rng};

use sanguine_lib::resources::{
    composition::{Density, Direction, CompositionOverlay},
    layout, border_coordinates, shapes::{Line, Point, Circle},
};

const RADIUS_MID: std::ops::RangeInclusive<i32> = 3_i32..=6_i32;
const RADIUS_HIGH: std::ops::RangeInclusive<i32> = 5_i32..=10_i32;
const RADIUS_FOCUS: std::ops::RangeInclusive<i32> = 10_i32..=20_i32;

pub fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();

    // Creates a baseline composition
    let mut comp = CompositionOverlay::new_empty(layout);
    comp.add_random_low(30, layout);
    comp.add_random_center(6, layout);
    comp.connect_centers(layout);
    comp.add_random_low(10, layout);

    let mut all_coords = border_coordinates::AllBorderCoordinates::new(layout, 10);
    all_coords.tesselate(layout);

    // Fills the gaps and edges in the baseline composition
    comp.retro_composition(layout);

    // Drawing of the Elements
    for row in 0..layout.rows {
        for col in 0..layout.columns {
            let mut rng = thread_rng();

            let mut radius = 0;

            match comp.0[row][col] {
                Density::Mid => radius = rng.gen_range(RADIUS_MID),
                Density::High => radius = rng.gen_range(RADIUS_HIGH),
                Density::Focus => radius = rng.gen_range(RADIUS_FOCUS),
                Density::Edge(_) => radius = rng.gen_range(RADIUS_MID),
                _ => (),
            }

            match comp.0[row][col] {
                Density::Empty => (),
                Density::Transition(Direction::UpDown) => {
                    // 1 -> 3 2 -> 0
                    for point in 0..10 {
                        let line = Line::new(
                            all_coords.0[row][col].0[2].0[point],
                            all_coords.0[row][col].0[0].0[9 - point],
                        );
                        graph.append(line.draw());
                    }
                }

                Density::Transition(Direction::LeftRight) => {
                    for point in 0..10 {
                        let line = Line::new(
                            all_coords.0[row][col].0[1].0[point],
                            all_coords.0[row][col].0[3].0[9 - point],
                        );
                        graph.append(line.draw());
                    }
                }

                Density::Low => {
                    // 1 -> 2 3 -> 0 oder 1 -> 0, 3 -> 2
                    let mut rng = thread_rng();

                    let side = rng.gen_range(0..=1);
                    let mut sec_side = 0;
                    let mut third_side = 0;

                    if side == 0 {
                        sec_side = 3;
                        third_side = 1;
                    } else if side == 1 {
                        sec_side = 0;
                        third_side = 3;
                    }

                    for point in 0..10 {
                        let line = Line::new(
                            all_coords.0[row][col].0[side].0[point],
                            all_coords.0[row][col].0[sec_side].0[9 - point],
                        );
                        graph.append(line.draw());
                    }

                    for point in 0..10 {
                        let line = Line::new(
                            all_coords.0[row][col].0[third_side].0[point],
                            all_coords.0[row][col].0[2].0[9 - point],
                        );
                        graph.append(line.draw());
                    }
                }

                Density::Edge(direction) => {
                    let center = Point::random_coordinate(
                        &layout.field_container[row as usize][col as usize],
                        radius * 2,
                    );
                    let circle = Circle::new(center, radius as f32);
                    graph.append(circle.draw());

                    let side = match direction {
                        Direction::Up => 0,
                        Direction::Down => 2,
                        Direction::Left => 3,
                        Direction::Right => 1,
                        _ => 5,
                    };

                    for point in 0..10 {
                        let prelim_line =
                            Line::new(all_coords.0[row][col].0[side].0[point], center);
                        let step = 1.0;

                        if let Some(endpoint) = circle.intersection(prelim_line, step) {
                            // println!("Point {:?}, EP {:?}", point, endpoint);
                            let line =
                                Line::new(all_coords.0[row][col].0[side].0[point], endpoint);
                            graph.append(line.draw());
                        };
                    }
                }

                Density::Corner(direction) => {
                    let mut first_side = 0;
                    let mut sec_side = 0;

                    match direction {
                        Direction::LeftDown => {
                            first_side = 1;
                            sec_side = 2;
                        }
                        Direction::LeftUp => {
                            first_side = 1;
                            sec_side = 0;
                        }
                        Direction::RightDown => {
                            first_side = 3;
                            sec_side = 2;
                        }
                        Direction::RightUp => {
                            first_side = 3;
                            sec_side = 0;
                        }
                        _ => (),
                    };
                    
                    for point in 0..10 {
                        let line = Line::new(
                            all_coords.0[row][col].0[first_side].0[point],
                            all_coords.0[row][col].0[sec_side].0[9 - point],
                        );
                        graph.append(line.draw());
                    }
                }

                Density::ThreeWay(direction) => {
                    let mut first_side = 0;
                    let mut mid_side = 0;
                    let mut last_side = 0;

                    match direction {
                        Direction::Left => {
                            first_side = 0;
                            mid_side = 3;
                            last_side = 2;
                        }
                        Direction::Up => {
                            first_side = 1;
                            mid_side = 2;
                            last_side = 3;
                        }
                        Direction::Down => {
                            first_side = 1;
                            mid_side = 0;
                            last_side = 3;
                        }
                        Direction::Right => {
                            first_side = 0;
                            mid_side = 1;
                            last_side = 2;
                        }
                        _ => (),
                    };
                    for point in 0..5 {
                        let line = Line::new(
                            all_coords.0[row][col].0[first_side].0[point],
                            all_coords.0[row][col].0[last_side].0[9 - point],
                        );
                        graph.append(line.draw());
                    }

                    for point in 5..10 {
                        let line = Line::new(
                            all_coords.0[row][col].0[first_side].0[point],
                            all_coords.0[row][col].0[mid_side].0[9 - (point - 5)],
                        );
                        graph.append(line.draw());
                    }

                    for point in 5..10 {
                        let line = Line::new(
                            all_coords.0[row][col].0[mid_side].0[point],
                            all_coords.0[row][col].0[last_side].0[9 - point],
                        );
                        graph.append(line.draw());
                    }
                }

                _ => {
                    let center = Point::random_coordinate(
                        &layout.field_container[row as usize][col as usize],
                        radius,
                    );

                    let circle = Circle::new(center, radius as f32);
                    graph.append(circle.draw());

                    for side in 0..4 {
                        for point in 0..10 {
                            let prelim_line =
                                Line::new(all_coords.0[row][col].0[side].0[point], center);
                            let step = 1.0;

                            if let Some(endpoint) = circle.intersection(prelim_line, step) {
                                // println!("Point {:?}, EP {:?}", point, endpoint);
                                let line =
                                    Line::new(all_coords.0[row][col].0[side].0[point], endpoint);
                                graph.append(line.draw());
                            } else {
                                println!("shit!");
                            };
                        }
                    }
                }
            }
        }
    }

    graph
}
