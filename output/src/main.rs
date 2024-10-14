use sanguine_lib::resources::{
    exclusion,
    layout::{self, Layout, LayoutType, Parameters, VoronoiType},
};
use svg::Document;

use chrono::Local;
use std::env;

mod work;
use work::{mush, star_burst, voronated_star_burst, voronoi, voronoi_simple};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let path = "exclusion.svg";
    let mut content = String::new();

    // voronoi::form_group();

    if let Some(exclusion) = exclusion::Exclusion::make_exclusion(path, &mut content) {
        for i in 0..10 {
            // let parameters = Parameters{ height: 1200, width: 600, margin: 2, rows: 0, columns: 0, layout_type: LayoutType::VoronoiBased(VoronoiType::Uniform(50)) };
            let parameters = Parameters {
                height: 1200,
                width: 1200,
                margin: 30,
                rows: 3,
                columns: 3,
                layout_type: LayoutType::GridBased(3, 3),
            };

            match layout::grid::Grid::new(parameters) {
                Ok(mut work) => {
                    let document = Document::new()
                        .set("viewBox", (0, 0, work.get_width(), work.get_height()))
                        .add(work.background())
                        .add(mush::form_group(work));

                    let local_time = Local::now();
                    let path = format!("nr_{:03}_{}.svg", i, local_time.format("%Y%m%d_%H%M%S"));
                    svg::save(path, &document).unwrap();
                }
                Err(err) => panic!("{:?}", err),
            }
        }
    };
    println!("None!");
}
