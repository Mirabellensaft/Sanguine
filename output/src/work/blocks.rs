use rand::{thread_rng, Rng};

use svg::{node::element::Group, Node};

use sanguine_lib::resources::{
    border_coordinates::all::AllBorderCoordinates,
    composition::{Composition, CompositionCenter, Density},
    layout::{grid::Grid, Layout},
    shapes::{circle::Circle, line::Line, point::Point},
};

pub fn form_group(work: Grid) -> Group {
    let mut graph = Group::new();
    let amount = 6;
    let mut all_coords = AllBorderCoordinates::new(&work, amount);

    all_coords.slight_chaos();
    let margin = 0;

    // Drawing of the Elements
    for row in 0..work.get_rows() {
        for col in 0..work.get_columns() {
            let mut rng = thread_rng();

            let field = &work.get_fields()[row as usize][col as usize];
            // println!(
            //     "field: x{}, y{}, width{}, height{}",
            //     field.x, field.y, field.column_width, field.row_height
            // );
            let point_1 = Point::new((field.x + margin) as f32, (field.y + margin) as f32);
            let point_2 = Point::new(
                (field.x + margin) as f32,
                (field.y + field.row_height - margin) as f32,
            );
            let point_3 = Point::new(
                (field.x + field.column_width - margin) as f32,
                (field.y + margin) as f32,
            );
            let point_4 = Point::new(
                (field.x + field.column_width - margin) as f32,
                (field.y + field.row_height - margin) as f32,
            );

            let line = Line::new(point_1, point_2);
            graph.append(line.draw());
            let line = Line::new(point_1, point_3);
            graph.append(line.draw());
            let line = Line::new(point_3, point_4);
            graph.append(line.draw());
            let line = Line::new(point_4, point_2);
            graph.append(line.draw());

            // Things to improve:
            // - Currently this works for 6 points on each side, needs to be changed so it works if the entire
            // number of points is even.
            // - current algorithm may favor a certain distribution of crossings, change it
            // - move to a lib when it's better and more general

            let mut points = Vec::new();
            let mut other_points = Vec::new();
            let mut min = 0;
            for side in 0..3 {
                for point in min..amount {
                    points.push(all_coords.0[row][col].0[side].0[point])
                }

                for other_side in (side + 1)..4 {
                    for point in min..min + (amount / 3) {
                        other_points.push(all_coords.0[row][col].0[other_side].0[point])
                    }
                }
                println!("len vec {}, {}", points.len(), other_points.len());
                for i in 0..points.len() {
                    let line = Line::new(points[i], other_points[i]);
                    graph.append(line.draw());
                }
                points.clear();
                other_points.clear();
                min += 2;
            }
        }
    }
    graph
}
