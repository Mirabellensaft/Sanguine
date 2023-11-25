use sanguine_lib::resources::border_coordinates::OneSide;
use svg::node;
use svg::Node;

use rand::{thread_rng, Rng};

use sanguine_lib::resources::{
    composition::{Density, Direction, CompositionOverlay},
    layout, border_coordinates, shapes::{Line, Point, Circle},
};
use svg::node::element::Group;

const RADIUS_MID: std::ops::RangeInclusive<i32> = 3_i32..=6_i32;
const RADIUS_HIGH: std::ops::RangeInclusive<i32> = 5_i32..=10_i32;
const RADIUS_FOCUS: std::ops::RangeInclusive<i32> = 10_i32..=20_i32;

pub fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();

    // Creates a baseline composition
    let mut comp = CompositionOverlay::new_empty(layout);
    comp.add_random_low(30, layout);
    comp.add_random_center(6, layout);
    comp.connect_centers();
    comp.add_random_low(10, layout);

    let mut all_coords = border_coordinates::AllBorderCoordinates::new(layout, 10);
    all_coords.tesselate();
    all_coords.slight_chaos();

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
                            all_coords.0[row][col].0[0].0[point],
                        );
                        graph.append(line.draw());
                    }
                }

                Density::Transition(Direction::LeftRight) => {
                    for point in 0..10 {
                        let line = Line::new(
                            all_coords.0[row][col].0[1].0[point],
                            all_coords.0[row][col].0[3].0[point],
                        );
                        graph.append(line.draw());
                    }
                }

                Density::Low => {
                    // 1 -> 2 3 -> 0 oder 1 -> 0, 3 -> 2
                    let mut rng = thread_rng();

                    let side = rng.gen_range(0..=1);
                    let mut second_side = 0;
                    let mut third_side = 0;
                    let mut dir = Direction::Up;
          

                    if side == 0 {
                        second_side = 3;
                        third_side = 1;
                        dir = Direction::RightUp;
                        
                    } else if side == 1 {
                        second_side = 0;
                        third_side = 3;
                        dir = Direction::RightDown;
                    }

                    let first = &all_coords.0[row][col].0[side];
                    let second = &all_coords.0[row][col].0[second_side];
                    nice_diagonal_lines(&mut graph, dir, first, second, 0, 10);

                    let first = &all_coords.0[row][col].0[third_side];
                    let second = &all_coords.0[row][col].0[2];
                    nice_diagonal_lines(&mut graph, dir, first, second, 0, 10);
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
        

                    match direction {
                        Direction::LeftDown => {
                            let second_side = &all_coords.0[row][col].0[1];
                            let first_side = &all_coords.0[row][col].0[2];
                            nice_diagonal_lines(&mut graph, direction, first_side, second_side, 0, 10)
                        }
                        Direction::LeftUp => {
                            let first_side = &all_coords.0[row][col].0[1];
                            let second_side = &all_coords.0[row][col].0[0];
                            nice_diagonal_lines(&mut graph, direction, &first_side, &second_side, 0, 10)
                        }
                        Direction::RightDown => {
                            let second_side = &all_coords.0[row][col].0[3];
                            let first_side = &all_coords.0[row][col].0[2];
                            nice_diagonal_lines(&mut graph, direction, &first_side, &second_side, 0, 10)
                        }
                        Direction::RightUp => {
                            let first_side = &all_coords.0[row][col].0[3];
                            let second_side = &all_coords.0[row][col].0[0];
                            nice_diagonal_lines(&mut graph, direction, &first_side, &second_side, 0, 10)
                        }
                        _ => (),
                    };
                }

                Density::ThreeWay(direction) => {
                  

                    match direction {
                        Direction::Left => {
                            let first_side = &all_coords.0[row][col].0[0];
                            let mid_side = &all_coords.0[row][col].0[3];
                            let last_side = &all_coords.0[row][col].0[2];

                            nice_three_ways(&mut graph, direction, &first_side, &mid_side, &last_side);
                        }
                        Direction::Up => {
                            let first_side = &all_coords.0[row][col].0[1];
                            let mid_side = &all_coords.0[row][col].0[2];
                            let last_side = &all_coords.0[row][col].0[3];

                            nice_three_ways(&mut graph, direction, &first_side, &mid_side, &last_side);
                        }
                        Direction::Down => {
                            let first_side = &all_coords.0[row][col].0[1];
                            let mid_side = &all_coords.0[row][col].0[0];
                            let last_side = &all_coords.0[row][col].0[3];

                            nice_three_ways(&mut graph, direction, &first_side, &mid_side, &last_side);
                        }
                        Direction::Right => {
                            let first_side = &all_coords.0[row][col].0[0];
                            let mid_side = &all_coords.0[row][col].0[1];
                            let last_side = &all_coords.0[row][col].0[2];

                            nice_three_ways(&mut graph, direction, &first_side, &mid_side, &last_side);
                        }
                        _ => (),
                    };
                },   

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

fn nice_diagonal_lines(graph: &mut Group, direction: Direction, first_side:  &OneSide, second_side: &OneSide, min: usize, max: usize) {

    match direction {
        Direction::LeftDown | Direction::RightUp => {
            for point in min..max {
                let line = Line::new(
                    first_side.0[point],
                    second_side.0[(max-1) - point],
                );
                graph.append(line.draw());
            }

        },
        Direction::RightDown | Direction::LeftUp | Direction::UpDown | Direction::LeftRight => {
            for point in min..max {
                let line = Line::new(
                    first_side.0[point],
                    second_side.0[point],
                );
                graph.append(line.draw());
            }
        },
        _ => {},
    }
                    
}

fn nice_three_ways(graph: &mut Group, direction: Direction, first_side:  &OneSide, mid_side: &OneSide, last_side: &OneSide) {

    match direction {
        Direction::Left => {
            for point in 0..5 {
                let line = Line::new(
                    first_side.0[point],
                    last_side.0[point],
                );
                graph.append(line.draw());
            }

            for point in 5..10 {
                let line = Line::new(
                    first_side.0[point],
                    mid_side.0[9-point],
                );
                graph.append(line.draw());
            }

            for point in 5..10 {
                let line = Line::new(
                    last_side.0[point],
                    mid_side.0[point],
                );
                graph.append(line.draw());
            }

        },
        Direction::Right => {
            for point in 5..10 {
                let line = Line::new(
                    first_side.0[point],
                    last_side.0[point],
                );
                graph.append(line.draw());
            }

            for point in 0..5 {
                let line = Line::new(
                    first_side.0[point],
                    mid_side.0[point],
                );
                graph.append(line.draw());
            }

            for point in 0..5 {
                let line = Line::new(
                    last_side.0[point],
                    mid_side.0[9-point],
                );
                graph.append(line.draw());
            }
        },
        Direction::Up => {
            for point in 0..5 {
                let line = Line::new(
                    first_side.0[point],
                    last_side.0[point],
                );
                graph.append(line.draw());
            }

            for point in 0..5 {
                let line = Line::new(
                    first_side.0[point+4],
                    mid_side.0[4-point],
                );
                graph.append(line.draw());
            }

            for point in 5..10 {
                let line = Line::new(
                    last_side.0[point],
                    mid_side.0[point],
                );
                graph.append(line.draw());
            }

        },
        Direction::Down => {
            for point in 5..10 {
                let line = Line::new(
                    first_side.0[point],
                    last_side.0[point],
                );
                graph.append(line.draw());
            }

            for point in 0..5 {
                let line = Line::new(
                    first_side.0[point],
                    mid_side.0[point],
                );
                graph.append(line.draw());
            }

            for point in 5..10 {
                let line = Line::new(
                    last_side.0[9-point],
                    mid_side.0[point],
                );
                graph.append(line.draw());
            }

        }
        _ => {},
    }
                    
}