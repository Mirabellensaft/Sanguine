use rand::Rng;
use sanguine_lib::resources::shapes::circle::Circle;
use sanguine_lib::resources::shapes::Shape;
use svg::node::element::Group;
use svg::Node;

use sanguine_lib::resources::{
    layout::grid::Grid,
    shapes::circloid::Circloid,
    shapes::{curve::Curve, line::Line, point::Point},
};

pub fn form_group(work: Grid) -> Group {
    let mut graph = Group::new();
    let rng = &mut rand::thread_rng();

    for row in 0..work.rows {
        for col in 0..work.columns {
            let amplitude_out = rng.gen_range(5.0..10.0);
            let frequency_out = rng.gen_range(1.2..3.0);
            let radius_out = rng.gen_range(150.0..180.0);
            let center_out = Point::new(
                work.container[row][col].center.x + rng.gen_range(-20.0..20.0),
                work.container[row][col].center.y + rng.gen_range(-20.0..20.0),
            );

            let amplitude_in = rng.gen_range(2.0..5.0);
            let frequency_in = rng.gen_range(1.2..3.0);
            let radius_in = rng.gen_range(40.0..65.0);

            let center_in = Point::new(
                center_out.x + rng.gen_range(-15.0..15.0),
                center_out.y + rng.gen_range(-15.0..15.0),
            );

            let circloid_in = Circloid::new(center_in, radius_in, amplitude_in, frequency_in);
            let circloid_out = Circloid::new(center_out, radius_out, amplitude_out, frequency_out);

            for i in 0..1000 {
                if i % 9 == 0 {
                    let line = Line::new(circloid_out.point_collection[i], center_in);
                    let circle = Circle::new(center_in, circloid_in.base_radius + 20.0);
                    let cp_inter = circle.intersection(line, 0.2).unwrap();

                    let cp_start = Point::new(
                        cp_inter.x + rng.gen_range(-3.0..3.0),
                        cp_inter.y + rng.gen_range(-3.0..3.0),
                    );

                    let line = Line::new(circloid_out.point_collection[i], center_out);
                    let circle = Circle::new(center_out, circloid_out.base_radius - 20.0);
                    let cp_inter = circle.intersection(line, 1.0).unwrap();

                    let cp_end = Point::new(
                        cp_inter.x + rng.gen_range(-3.0..3.0),
                        cp_inter.y + rng.gen_range(-3.0..3.0),
                    );

                    let curve = Curve::new(
                        circloid_in.point_collection[i],
                        cp_start,
                        cp_end,
                        circloid_out.point_collection[i],
                    );
                    graph.append(curve.draw());
                }
            }
        }
    }
    graph
}
