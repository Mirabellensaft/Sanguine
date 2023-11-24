use svg::Document;

use sanguine_lib::resources::layout;

use std::env;

mod work;
use work::{star_burst, star_burst_comp, tester};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    for i in 0..20 {
        let work = layout::Format::new(1200, 600, 2, 20, 10);
        let document = Document::new()
            .set("viewBox", (0, 0, work.width, work.height))
            .add(layout::backgound(&work))
            .add(star_burst_comp::form_group(&work));

        let path = format!("image_{}.svg", i);
        svg::save(path, &document).unwrap();
    }
}
