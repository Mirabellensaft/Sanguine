use sanguine_lib::resources::{exclusion, layout};
use svg::Document;

use std::env;

mod work;
use work::{star_burst, tester};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let path = "exclusion.svg";
    let mut content = String::new();

    if let Some(exclusion) = exclusion::Exclusion::make_exclusion(path, &mut content) {
        println!("Some");
        for i in 0..1 {
            let work = layout::Format::new(1200, 600, 2, 8, 4);
            let document = Document::new()
                .set("viewBox", (0, 0, work.width, work.height))
                .add(layout::backgound(&work))
                .add(tester::form_group(&work, &exclusion));

            let path = format!("image_{}.svg", i);
            svg::save(path, &document).unwrap();
        }
    };
    println!("None");
}
