use super::lines;
use rand::thread_rng;
use sanguine_lib::resources::{
    border_coordinates::cell_border::CellBorderCoords,
    composition::{Density, Direction},
    layout::voronoi::Cell,
    shapes::{circle::Circle, line::Line},
};
use svg::{node::element::Group, Node};

pub fn everything(
    cell: &Cell,
    border_coordinates: &CellBorderCoords,
    radius: i32,
    mut graph: Group,
) -> Group {
    let mut rng = thread_rng();

    match &cell.density {
        Density::Empty => {
            let circle = Circle::new(cell.center, 20.0);
            graph.append(circle.draw());
        }
        Density::Transition(Direction::Lines(lines)) => {
            println!("draw transition");
            println!("touch line: {:?}", lines);

            let mut shorter_line = 1;
            // let mut indices = Vec::new();
            // for border in 0..cell.border_lines.len() {
            //     for line in lines {
            //         if cell.border_lines[border].equal(line.0) {}
            //         indices.push(border)
            //     }
            // }

            if border_coordinates.0[lines[0].1].0.len() < border_coordinates.0[lines[1].1].0.len() {
                shorter_line = 0_usize;
            }
            for point in 0..border_coordinates.0[lines[shorter_line].1].0.len() {
                let line = Line::new(
                    border_coordinates.0[lines[0].1].0[point],
                    border_coordinates.0[lines[1].1].0[point],
                );
                graph.append(line.draw());
            }
        }

        Density::Lopsided(Direction::Lines(lines)) => {
            let circle = Circle::new(cell.center, radius as f32);
            graph.append(circle.draw());

            for side in &border_coordinates.0 {
                for line in lines {
                    println!("new line/side");
                    if side.equal(&line.0) {
                        println!("draw");
                        lines::to_circle(&mut graph, &side, &circle, 0, side.0.len());
                    }
                    println!("free side");
                }
            }
        }

        Density::Low => {
            if border_coordinates.0.len() % 2 == 0 {
                for border in &border_coordinates.0 {}
            };
        }

        Density::Edge(direction) => {
            // let center = Point::random_coordinate(field, radius * 2);

            // let side = match direction {
            //     Direction::Up => 0,
            //     Direction::Down => 2,
            //     Direction::Left => 3,
            //     Direction::Right => 1,
            //     _ => 5,
            // };

            // let circle = Circle::new(center, radius as f32);
            // graph.append(circle.draw());

            // let the_side = &border_coordinates.0[side];
            // lines::to_circle(&mut graph, the_side, &circle, 0, 10);
        }

        Density::ThreeWay(lines) => {}

        _ => {
            let circle = Circle::new(cell.center, radius as f32);
            graph.append(circle.draw());

            // for side in 0..cell.border_lines.len() {
            //     let the_side = &border_coordinates.0[side];
            for the_side in &border_coordinates.0 {
                lines::to_circle(&mut graph, the_side, &circle, 0, the_side.0.len());
            }
        }
    }

    graph
}
