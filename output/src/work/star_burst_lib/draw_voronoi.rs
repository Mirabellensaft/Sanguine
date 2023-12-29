use super::lines;
use rand::thread_rng;
use sanguine_lib::resources::{
    border_coordinates::cell_border::CellBorderCoords,
    composition::{Density, Direction},
    layout::voronoi::Cell,
    shapes::{circle::Circle, line::Line, Shape},
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
            let circle = Circle::new(cell.center, 3.0);
            graph.append(circle.debug_draw("red"));
        }
        Density::Transition(Direction::Lines(lines)) => {
            println!("draw transition");
            println!("touch line: {:?}", lines);

            let mut shorter_line = 1;
            let mut longer_line = 1;
            // let mut indices = Vec::new();
            // for border in 0..cell.border_lines.len() {
            //     for line in lines {
            //         if cell.border_lines[border].equal(line.0) {}
            //         indices.push(border)
            //     }
            // }

            if border_coordinates.0[lines[0].1].0.len() < border_coordinates.0[lines[1].1].0.len() {
                shorter_line = 0_usize;
                longer_line = 1_usize;
            }

            let diff = border_coordinates.0[lines[0].1]
                .0
                .len()
                .abs_diff(border_coordinates.0[lines[1].1].0.len());

            for point in 0..border_coordinates.0[lines[shorter_line].1].0.len() {
                let line = Line::new(
                    border_coordinates.0[lines[0].1].0[point],
                    border_coordinates.0[lines[1].1].0[point],
                );
                graph.append(line.draw());
            }

            for point in border_coordinates.0[lines[shorter_line].1].0.len()
                ..border_coordinates.0[lines[longer_line].1].0.len()
            {
                let circle = Circle::new(cell.center, radius as f32);
                graph.append(circle.draw());

                let prelim_line = Line::new(
                    border_coordinates.0[lines[longer_line].1].0[point],
                    circle.center,
                );
                let step = 1.0;

                if let Some(endpoint) = circle.intersection(prelim_line, step) {
                    // println!("Point {:?}, EP {:?}", point, endpoint);
                    let line = Line::new(
                        border_coordinates.0[lines[longer_line].1].0[point],
                        endpoint,
                    );
                    graph.append(line.draw());
                    // println!("YEAH");
                } else {
                    //debug
                    graph.append(prelim_line.draw());
                    // println!("shit!\n");
                };
            }
        }

        Density::Lopsided(Direction::Lines(lines)) => {
            let circle = Circle::new(cell.center, radius as f32);
            graph.append(circle.draw());

            for i in 0..lines.len() {
                println!("draw");
                lines::to_circle(
                    &mut graph,
                    &border_coordinates.0[lines[i].1],
                    &circle,
                    0,
                    border_coordinates.0[lines[i].1].0.len(),
                );
            }
            // println!("free side");
        }

        Density::Low => {
            if border_coordinates.0.len() % 2 == 0 {
                for i in (1..=border_coordinates.0.len()).step_by(2) {
                    if border_coordinates.0[i - 1].0.len() == border_coordinates.0[i].0.len() {
                        for point in 0..border_coordinates.0[i - 1].0.len() {
                            let line = Line::new(
                                border_coordinates.0[i - 1].0[point],
                                border_coordinates.0[1].0[point],
                            );
                            graph.append(line.draw());
                            continue;
                        }
                    }
                }
            } else {
                let circle = Circle::new(cell.center, 3.0);
                graph.append(circle.debug_draw("purple"));
            }
        }

        Density::Edge(Direction::Lines(lines)) => {
            let circle = Circle::new(cell.center, radius as f32);
            graph.append(circle.draw());

            lines::to_circle(
                &mut graph,
                &border_coordinates.0[lines[0].1],
                &circle,
                0,
                border_coordinates.0[lines[0].1].0.len(),
            );
        }

        Density::ThreeWay(Direction::Lines(lines)) => {
            let line_0 = border_coordinates.0[lines[0].1].0.len();
            let line_1 = border_coordinates.0[lines[1].1].0.len();
            let line_2 = border_coordinates.0[lines[2].1].0.len();

            println!("length: {}, {}, {}", line_0, line_1, line_2);
            if line_0 == line_1 && line_0 == line_2 {
                for i in 0..(border_coordinates.0[lines[0].1].0.len() / 2) {
                    let line = Line::new(
                        border_coordinates.0[lines[0].1].0[i],
                        border_coordinates.0[lines[1].1].0[i],
                    );

                    graph.append(line.draw());
                }

                for i in (border_coordinates.0[lines[0].1].0.len() / 2)
                    ..border_coordinates.0[lines[0].1].0.len()
                {
                    let line = Line::new(
                        border_coordinates.0[lines[0].1].0[i],
                        border_coordinates.0[lines[2].1].0[i],
                    );

                    graph.append(line.draw());
                }
                for i in 0..(border_coordinates.0[lines[2].1].0.len() / 2) {
                    let line = Line::new(
                        border_coordinates.0[lines[2].1].0[i],
                        border_coordinates.0[lines[1].1].0
                            [i + (border_coordinates.0[lines[2].1].0.len() / 2)],
                    );

                    graph.append(line.draw());
                }
            } else {
                let circle = Circle::new(cell.center, 3.0);
                graph.append(circle.debug_draw("green"));
            }
        }

        _ => {
            let circle = Circle::new(cell.center, radius as f32);
            graph.append(circle.draw());

            // for side in 0..cell.border_lines.len() {
            //     let the_side = &border_coordinates.0[side];
            for side in &border_coordinates.0 {
                lines::to_circle(&mut graph, side, &circle, 0, side.0.len());
            }
        }
    }

    graph
}
