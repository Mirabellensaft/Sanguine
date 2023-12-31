use sanguine_lib::resources::{
    border_coordinates::one_side::OneSide,
    composition::Direction,
    shapes::{circle::Circle, line::Line, Shape},
};
use svg::{node::element::Group, Node};

pub fn to_circle(graph: &mut Group, side: &OneSide, circle: &Circle, min: usize, max: usize) {
    for point in min..max {
        let prelim_line = Line::new(side.0[point], circle.center);
        let step = 1.0;

        if let Some(endpoint) = circle.intersection(prelim_line, step) {
            // println!("Point {:?}, EP {:?}", point, endpoint);
            let line = Line::new(side.0[point], endpoint);
            graph.append(line.draw());
            // println!("YEAH");
        } else {
            //debug
            graph.append(prelim_line.draw());
            // println!("shit!\n");
        };
    }
}

pub fn diagonal(
    graph: &mut Group,
    direction: &Direction,
    first_side: &OneSide,
    second_side: &OneSide,
    min: usize,
    max: usize,
) {
    match direction {
        Direction::LeftDown | Direction::RightUp => {
            for point in min..max {
                let line = Line::new(first_side.0[point], second_side.0[(max - 1) - point]);
                graph.append(line.draw());
            }
        }
        Direction::RightDown | Direction::LeftUp | Direction::UpDown | Direction::LeftRight => {
            for point in min..max {
                let line = Line::new(first_side.0[point], second_side.0[point]);
                graph.append(line.draw());
            }
        }
        _ => {}
    }
}
