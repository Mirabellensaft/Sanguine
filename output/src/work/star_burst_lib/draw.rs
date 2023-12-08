use super::{lines, threeways};
use rand::{thread_rng, Rng};
use sanguine_lib::resources::{
    border_coordinates::BorderCoordinates,
    composition::grid::{Density, Direction},
    layout::Field,
    shapes::{circle::Circle, line::Line, point::Point},
};
use svg::{node::element::Group, Node};

pub fn everything(
    density: Density,
    border_coordinates: &BorderCoordinates,
    field: &Field,
    radius: i32,
    mut graph: Group,
) -> Group {
    let mut rng = thread_rng();
    match density {
        Density::Empty => (),
        Density::Transition(Direction::UpDown) => {
            // 1 -> 3 2 -> 0
            for point in 0..10 {
                let line = Line::new(
                    border_coordinates.0[2].0[point],
                    border_coordinates.0[0].0[point],
                );
                graph.append(line.draw());
            }
        }

        Density::Transition(Direction::LeftRight) => {
            for point in 0..10 {
                let line = Line::new(
                    border_coordinates.0[1].0[point],
                    border_coordinates.0[3].0[point],
                );
                graph.append(line.draw());
            }
        }

        Density::Low => {
            // 1 -> 2 3 -> 0 oder 1 -> 0, 3 -> 2
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

            let first = &border_coordinates.0[side];
            let second = &border_coordinates.0[second_side];
            lines::diagonal(&mut graph, dir, first, second, 0, 10);

            let first = &border_coordinates.0[third_side];
            let second = &border_coordinates.0[2];
            lines::diagonal(&mut graph, dir, first, second, 0, 10);
        }

        Density::Edge(direction) => {
            let center = Point::random_coordinate(field, radius * 2);

            let side = match direction {
                Direction::Up => 0,
                Direction::Down => 2,
                Direction::Left => 3,
                Direction::Right => 1,
                _ => 5,
            };

            let circle = Circle::new(center, radius as f32);
            graph.append(circle.draw());

            let the_side = &border_coordinates.0[side];
            lines::to_circle(&mut graph, the_side, &circle, 0, 10);
        }

        Density::Corner(direction) => {
            match direction {
                Direction::LeftDown => {
                    let second_side = &border_coordinates.0[1];
                    let first_side = &border_coordinates.0[2];
                    lines::diagonal(&mut graph, direction, first_side, second_side, 0, 10)
                }
                Direction::LeftUp => {
                    let first_side = &border_coordinates.0[1];
                    let second_side = &border_coordinates.0[0];
                    lines::diagonal(&mut graph, direction, &first_side, &second_side, 0, 10)
                }
                Direction::RightDown => {
                    let second_side = &border_coordinates.0[3];
                    let first_side = &border_coordinates.0[2];
                    lines::diagonal(&mut graph, direction, &first_side, &second_side, 0, 10)
                }
                Direction::RightUp => {
                    let first_side = &border_coordinates.0[3];
                    let second_side = &border_coordinates.0[0];
                    lines::diagonal(&mut graph, direction, &first_side, &second_side, 0, 10)
                }
                _ => (),
            };
        }

        Density::ThreeWay(direction) => {
            match direction {
                Direction::Left => {
                    let first_side = &border_coordinates.0[0];
                    let mid_side = &border_coordinates.0[3];
                    let last_side = &border_coordinates.0[2];

                    threeways::draw(&mut graph, direction, &first_side, &mid_side, &last_side);
                }
                Direction::Up => {
                    let first_side = &border_coordinates.0[1];
                    let mid_side = &border_coordinates.0[2];
                    let last_side = &border_coordinates.0[3];

                    threeways::draw(&mut graph, direction, &first_side, &mid_side, &last_side);
                }
                Direction::Down => {
                    let first_side = &border_coordinates.0[1];
                    let mid_side = &border_coordinates.0[0];
                    let last_side = &border_coordinates.0[3];

                    threeways::draw(&mut graph, direction, &first_side, &mid_side, &last_side);
                }
                Direction::Right => {
                    let first_side = &border_coordinates.0[0];
                    let mid_side = &border_coordinates.0[1];
                    let last_side = &border_coordinates.0[2];

                    threeways::draw(&mut graph, direction, &first_side, &mid_side, &last_side);
                }
                _ => (),
            };
        }

        _ => {
            let center = Point::random_coordinate(field, radius);

            let circle = Circle::new(center, radius as f32);
            graph.append(circle.draw());

            for side in 0..4 {
                let the_side = &border_coordinates.0[side];
                lines::to_circle(&mut graph, the_side, &circle, 0, 10);
            }
        }
    }

    graph
}
