use svg::Document;

use sanguine_lib::resources::layout;

use std::env;

mod work;
use work::{star_burst, tester, star_burst_comp};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    for i in 0..20 {
        let work = layout::Format::new(600, 300, 2, 10, 5);
        let document = Document::new()
            .set("viewBox", (0, 0, work.width, work.height))
            .add(star_burst_comp::form_group(&work));
        let path = format!("image_{}.svg", i);
        svg::save(path, &document).unwrap();
    }
}
