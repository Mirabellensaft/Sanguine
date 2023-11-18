use svg::node;
use svg::Node;

use rand::{thread_rng, Rng};

use sanguine_lib::resources::{
    layout::{self, Orientation},
    random_numbers,
    shapes::{self, Point},
};

pub fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();

    let mut vec: Vec<[[Point; 10]; 4]> = Vec::new();
    let mut counter = 0;
    let mut group: [[Point; 10]; 4] = [[shapes::Point::new(0.0, 0.0); 10]; 4];
    let sides: [Orientation; 4] = [
        Orientation::Left,
        Orientation::Top,
        Orientation::Right,
        Orientation::Bottom,
    ];

    for row in 0..layout.rows {
        for col in 0..layout.columns {
            if row == 0 && col == 0 {
                for i in 0..4 {
                    group[i] = random_numbers::tesselating_coordinates_on_border(
                        &layout.field_container[row as usize][col as usize],
                        sides[i],
                    );
                }
                vec.insert(counter, group);
                counter += 1;
            } else if row == 0 && col != 0 {
                group[0] = vec[counter - 1][2];

                for i in 1..4 {
                    group[i] = random_numbers::tesselating_coordinates_on_border(
                        &layout.field_container[row as usize][col as usize],
                        sides[i],
                    );
                }
                vec.insert(counter, group);
                counter += 1;
            } else if row != 0 && col == 0 {
                for i in 0..4 {
                    group[i] = random_numbers::tesselating_coordinates_on_border(
                        &layout.field_container[row as usize][col as usize],
                        sides[i],
                    );
                }

                group[1] = vec[counter - layout.columns as usize][3];
                vec.insert(counter, group);
                counter += 1;
            } else if row != 0 && col != 0 {
                for i in 2..4 {
                    group[i] = random_numbers::tesselating_coordinates_on_border(
                        &layout.field_container[row as usize][col as usize],
                        sides[i],
                    );
                }

                group[0] = vec[counter - 1][2];
                group[1] = vec[counter - layout.columns as usize][3];
                vec.insert(counter, group);
                counter += 1;
            }
        }
    }

    for row in 0..layout.rows {
        for col in 0..layout.columns {
            let mut rng = thread_rng();
            let radius = rng.gen_range(3..=10);

            let center = random_numbers::coordinate(
                &layout.field_container[row as usize][col as usize],
                radius,
            );

            let circle = shapes::Circle::new(center, radius as f32);
            graph.append(circle.draw());

            // let coordinates = random_numbers::coordinates_on_border(&layout.field_container[row as usize][col as usize]);

            for i in 0..vec.len() {
                for side in 0..4 {
                    for point in 0..10 {
                        let mut line = shapes::Line::new(vec[i][side][point], center);
                        println!("point_1 {:?}, point_2 {:?}", vec[i][side][point], center);

                        // find endpoint on circle

                        let mut endpoint = shapes::Point::new(0.0, 0.0);

                        if center.x > vec[i][side][point].x {
                            let range: std::ops::Range<i32> = std::ops::Range {
                                start: vec[i][side][point].x as i32,
                                end: center.x as i32,
                            };
                            println!("1: range {:?}", range);
                            for j in range {
                                let p = line.return_point_on_line(j as f32 - 0.5);
                                // println!("point {:?}", point);
                                // println!("j {:?}\n", j);

                                if circle.contains(p) {
                                    endpoint = line.return_point_on_line(j as f32);
                                    println!("\n1: enpoint point {:?\n}", endpoint);
                                    line = shapes::Line::new(vec[i][side][point], endpoint);
                                    graph.append(line.draw());
                                    println!("endlich drin");
                                    break;
                                } else {
                                    println!("noch nicht");
                                };
                            }
                        } else {
                            let range: std::ops::Range<i32> = std::ops::Range {
                                start: center.x as i32,
                                end: vec[i][side][point].x as i32,
                            };
                            println!("2: range {:?}", range);
                            for j in range {
                                let p = line.return_point_on_line(j as f32);
                                if circle.contains(p) == false {
                                    endpoint = line.return_point_on_line(j as f32 - 0.5);
                                    println!("\n2: endpoint point {:?}", endpoint);
                                    line = shapes::Line::new(vec[i][side][point], endpoint);
                                    graph.append(line.draw());
                                    println!("nicht mehr");
                                    break;
                                } else {
                                    println!("noch drin");
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    graph
}
