use sanguine_lib::resources::{
    border_coordinates::OneSide,
    composition::Direction,
    shapes::{line::Line, Shape},
};
use svg::{node::element::Group, Node};

pub fn draw(
    graph: &mut Group,
    direction: Direction,
    first_side: &OneSide,
    mid_side: &OneSide,
    last_side: &OneSide,
) {
    match direction {
        Direction::Left => {
            for point in 0..5 {
                let line = Line::new(first_side.0[point], last_side.0[point]);
                graph.append(line.draw());
            }

            for point in 5..10 {
                let line = Line::new(first_side.0[point], mid_side.0[9 - point]);
                graph.append(line.draw());
            }

            for point in 5..10 {
                let line = Line::new(last_side.0[point], mid_side.0[point]);
                graph.append(line.draw());
            }
        }

        Direction::Right => {
            for point in 5..10 {
                let line = Line::new(first_side.0[point], last_side.0[point]);
                graph.append(line.draw());
            }

            for point in 0..5 {
                let line = Line::new(first_side.0[point], mid_side.0[point]);
                graph.append(line.draw());
            }

            for point in 0..5 {
                let line = Line::new(last_side.0[point], mid_side.0[9 - point]);
                graph.append(line.draw());
            }
        }

        Direction::Up => {
            for point in 0..5 {
                let line = Line::new(first_side.0[point], last_side.0[point]);
                graph.append(line.draw());
            }

            for point in 0..5 {
                let line = Line::new(first_side.0[point + 4], mid_side.0[4 - point]);
                graph.append(line.draw());
            }

            for point in 5..10 {
                let line = Line::new(last_side.0[point], mid_side.0[point]);
                graph.append(line.draw());
            }
        }

        Direction::Down => {
            for point in 5..10 {
                let line = Line::new(first_side.0[point], last_side.0[point]);
                graph.append(line.draw());
            }

            for point in 0..5 {
                let line = Line::new(first_side.0[point], mid_side.0[point]);
                graph.append(line.draw());
            }

            for point in 5..10 {
                let line = Line::new(last_side.0[9 - point], mid_side.0[point]);
                graph.append(line.draw());
            }
        }
        _ => {}
    }
}
