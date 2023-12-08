use sanguine_lib::resources::{exclusion, layout};
use svg::Document;

use std::env;
use chrono::{DateTime, Local};

mod work;
use work::{star_burst, tester, voronoi};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let path = "exclusion.svg";
    let mut content = String::new();


    // voronoi::form_group();

    if let Some(exclusion) = exclusion::Exclusion::make_exclusion(path, &mut content) {
        println!("Some");
        for i in 0..5 {
            let work = layout::Grid::new(1200, 600, 2, 8, 4);
            let document = Document::new()
                .set("viewBox", (0, 0, work.width, work.height))
                .add(layout::background(&work))
                .add(voronoi::form_group(&work));

            let local_time = Local::now();
            let path = format!("nr_{:03}_{}.svg", i, local_time.format("%Y%m%d_%H%M%S"));
            svg::save(path, &document).unwrap();
        }
    };
    println!("None");
}
