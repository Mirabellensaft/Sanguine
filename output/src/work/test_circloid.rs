use svg::node::element::Group;
use svg::Node;

use sanguine_lib::resources::{layout::grid::Grid, shapes::circloid::Circloid};

pub fn form_group(work: Grid) -> Group {
    let mut graph = Group::new();
    let mut amplitude = 2.0;
    let mut frequency = 2.0;
    let mut radius = 20.0;

    println!(
        "rows: {}, col: {}, width: {}, height: {}",
        work.rows, work.columns, work.width, work.height
    );

    for row in 0..work.rows {
        amplitude += 1.0;
        radius = 20.0;
        for col in 0..work.columns {
            let center = work.container[row][col].center;

            println!("f: {}, a: {}, r: {}", frequency, amplitude, radius);
            println!("row: {}, col: {}", row, col);

            let circloid = Circloid::new(center, radius, amplitude, frequency);

            graph.append(circloid.draw());
            radius += 5.0
        }
    }

    // for row in 0..work.rows {
    //     for col in 0..work.columns {
    //         let center = work.container[row][col].center;

    //         println!("f: {}, a: {}, r: {}", frequency, amplitude, radius);
    //         println!("row: {}, col: {}", row, col);

    //         let circloid = Circloid::new(center, radius, amplitude, frequency);

    //         graph.append(circloid.draw());
    //         radius += 10.0;
    //     }
    // }

    // for row in 0..work.rows {
    //     amplitude += 1.0;
    //     frequency = 1.4;
    //     for col in 0..work.columns {
    //         frequency += 0.2;

    //         let center = work.container[row][col].center;
    //         let radius = 80.0;
    //         println!("f: {}, a: {}, r: {}", frequency, amplitude, radius);
    //         println!("row: {}, col: {}", row, col);

    //         let circloid = Circloid::new(center, radius, amplitude, frequency);

    //         graph.append(circloid.draw());
    //     }
    // }

    graph
}
