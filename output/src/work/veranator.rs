use rand::{seq::SliceRandom, thread_rng, Rng};

use svg::{node::element::Group, Node};

use sanguine_lib::resources::{
    border_coordinates::all::AllBorderCoordinates,
    layout::{grid::Grid, Layout},
    shapes::{line::Line, point::Point},
};

pub fn form_group(work: Grid) -> Group {
    let mut graph = Group::new();
    // let amount = 6;
    // let mut all_coords = AllBorderCoordinates::new(&work, amount);

    // all_coords.slight_chaos();
    let margin = 0;

    // Drawing of the Elements
    for row in 0..work.get_rows() {
        for col in 0..work.get_columns() {
            println!("row {}", row);
            println!("col {}", col);
            let tile = &work.get_tiles()[row as usize][col as usize];
            // println!(
            //     "tile: {:?} tile: x{}, y{}, width{}, height{}",
            //     tile, tile.x, tile.y, tile.width, tile.height
            // );
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

            println!("{:?}, {:?}, {:?}, {:?}", point_1, point_2, point_3, point_4);

            for i in 0..15 {
                let a = i as f32 * 10.0;
                let line = Line::new(
                    Point {
                        x: point_1.x + a,
                        y: point_1.y + a,
                    },
                    Point {
                        x: point_2.x + a,
                        y: point_2.y - a,
                    },
                );

                graph.append(line.draw());

                let line = Line::new(
                    Point {
                        x: point_1.x + a,
                        y: point_1.y + a,
                    },
                    Point {
                        x: point_3.x - a,
                        y: point_3.y + a,
                    },
                );

                graph.append(line.draw());
                let line = Line::new(
                    Point {
                        x: point_3.x - a,
                        y: point_3.y + a,
                    },
                    Point {
                        x: point_4.x - a,
                        y: point_4.y - a,
                    },
                );

                graph.append(line.draw());

                let line = Line::new(
                    Point {
                        x: point_4.x - a,
                        y: point_4.y - a,
                    },
                    Point {
                        x: point_2.x + a,
                        y: point_2.y - a,
                    },
                );
                graph.append(line.draw());
            }

            // Things to improve:
            // - Currently this works for 6 points on each side, needs to be changed so it works if the entire
            // number of points is even.
            // - current algorithm may favor a certain distribution of crossings, change it
            // - move to a lib when it's better and more general
        }
    }
    graph
}
