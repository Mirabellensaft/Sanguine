use rand::{seq::SliceRandom, thread_rng, Rng};

use svg::{node::element::Group, Node};

use sanguine_lib::resources::{
    border_coordinates::all::AllBorderCoordinates,
    layout::{grid::Grid, Layout},
    shapes::{line::Line, point::Point},
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
            let tile = &work.get_tiles()[row as usize][col as usize];
            println!(
                "tile: x{}, y{}, width{}, height{}",
                tile.x, tile.y, tile.width, tile.height
            );
            let point_1 = Point::new((tile.x + margin) as f32, (tile.y + margin) as f32);
            let point_2 = Point::new(
                (tile.x + margin) as f32,
                (tile.y + tile.height - margin) as f32,
            );
            let point_3 = Point::new(
                (tile.x + tile.width - margin) as f32,
                (tile.y + margin) as f32,
            );
            let point_4 = Point::new(
                (tile.x + tile.width - margin) as f32,
                (tile.y + tile.height - margin) as f32,
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

                other_points.shuffle(&mut thread_rng());
                points.shuffle(&mut thread_rng());
                for i in 0..points.len() / 2 {
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
