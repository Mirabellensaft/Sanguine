use svg::Document;

use sanguine_lib::resources::layout;

mod work;
use work::{star_burst, tester};

fn main() {
    let work = layout::Format::new(300, 300, 2, 5, 5);
    let document = Document::new()
        .set("viewBox", (0, 0, work.width, work.height))
        .add(star_burst::form_group(&work));

    svg::save("image.svg", &document).unwrap();
}
