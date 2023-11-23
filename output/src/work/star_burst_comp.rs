use svg::node;
use svg::Node;

use rand::{thread_rng, Rng};

use sanguine_lib::resources::{
    layout,
    random_numbers,
    shapes,
    composition::{self, Density},
};

pub fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();

    let mut comp = composition::CompositionOverlay::new_flat(layout);
    comp.add_random_emty(15, layout);
    comp.add_random_center(3, layout);
    comp.add_random_emty(5, layout);
   

    let mut all_coords = random_numbers::all_border_coordinates(layout);
    let mut counter = 0;

    for row in 0..layout.rows {
        for col in 0..layout.columns {
            if row == 0 && col != 0 {
                all_coords[row][col][1] = all_coords[row][col - 1][3];
            } else if row != 0 && col == 0 {
                all_coords[row][col][0] = all_coords[row - 1][col][2];
            } else if row != 0 && col != 0 {
                all_coords[row][col][1] = all_coords[row][col - 1][3];
                all_coords[row][col][0] = all_coords[row - 1][col][2];
            }
        }
    }

    // let coordinates = random_numbers::coordinates_on_border(&layout.field_container[row as usize][col as usize]);

    for row in 0..layout.rows {
        for col in 0..layout.columns {
            let mut rng = thread_rng();

            let mut radius = 0;

            match comp.0[row][col] {
                Density::Low => radius = rng.gen_range(3..=6),
                Density::Mid => radius = rng.gen_range(5..=10),
                Density::High => radius = rng.gen_range(10..=20),
                Density::Empty => (),
            }

            match comp.0[row][col] {
                Density::Empty => {
                    // 1 -> 2 3 -> 0 oder 1 -> 0, 3 -> 2
                    let mut rng = thread_rng();

                    let side = rng.gen_range(0..=1);
                    let mut sec_side = 0;
                    let mut third_side = 0;
                    let mut fourth_side = 0;

                    if side == 0 {
                        sec_side = 3;
                        third_side = 1;
                    } else if side == 1 {
                        sec_side = 0;
                        third_side = 3;
                    }

                    for point in 0..10 {
                        let line =
                            shapes::Line::new(all_coords[row][col][side][point], all_coords[row][col][sec_side][9 - point]);
                            graph.append(line.draw());
                    }

                    for point in 0..10 {
                        let line =
                            shapes::Line::new(all_coords[row][col][third_side][point], all_coords[row][col][2][9 - point]);
                            graph.append(line.draw());
                    }  
                },
                _ => {
                    let center = random_numbers::coordinate(
                        &layout.field_container[row as usize][col as usize],
                        radius,
                    );
        
                    let circle = shapes::Circle::new(center, radius as f32);
                    graph.append(circle.draw());
        
                    for side in 0..4 {
                        for point in 0..10 {
                            // print!(
                            //     "{}, {}, start: {:?}, center:{:?} \n",
                            //     side, point, all_coords[row][col][side][point], center
                            // );
                            let mut prelim_line =
                                shapes::Line::new(all_coords[row][col][side][point], center);
                            let step = 1.0;
        
                            if let Some(endpoint) = circle.intersection(prelim_line, step) {
                                // println!("Point {:?}, EP {:?}", point, endpoint);
                                let line = shapes::Line::new(all_coords[row][col][side][point], endpoint);
                                graph.append(line.draw());
                            };
                        }
                    }
                }
            }  
        }
    }

    graph
}
