// fn form_group_test(layout: &layout::Format) -> node::element::Group {
//     let mut graph = node::element::Group::new();
//     let origin = Point::new(0.0, 0.0);
//     for row in 0..layout.rows {
//         for col in 0..layout.columns {
//             for row in 0..layout.rows {
//                 for col in 0..layout.columns {

//                     let mut rng = thread_rng();
//                     let radius = rng.gen_range(3..=10);

//                     let center = random_numbers::coordinate(&layout.field_container[row as usize][col as usize], radius);

//                     let circle =shapes::Circle::new(center, radius as f32);
//                     graph.append(circle.draw());

//                     // let coordinates = random_numbers::coordinates_on_border(&layout.field_container[row as usize][col as usize]);

//                     for i in 0..40 {

//                         let mut line = shapes::Line::new(coordinates[i], center);
//                         println!("point_1 {:?}, point_2 {:?}", coordinates[i], center);

//                         // find endpoint on circle

//                         let mut endpoint = shapes::Point::new(0.0, 0.0);

//                         if center.x > coordinates[i].x {

//                             let range: std::ops::Range<i32> = std::ops::Range {
//                                 start: coordinates[i].x as i32,
//                                 end: center.x as i32,

//                             };
//                             println!("1: range {:?}", range);
//                             for j in  range {
//                                 let point = line.return_point_on_line(j as f32 - 0.5);
//                                 // println!("point {:?}", point);
//                                 // println!("j {:?}\n", j);

//                                 if circle.contains(point) {
//                                     endpoint = line.return_point_on_line(j as f32);
//                                     println!("\n1: enpoint point {:?\n}", endpoint);
//                                     line = shapes::Line::new(coordinates[i], endpoint);
//                                     graph.append(line.draw());
//                                     println!("endlich drin");
//                                     break;

//                                 } else {
//                                     println!("noch nicht");

//                                 };
//                             }

//                         } else {

//                             let range: std::ops::Range<i32> = std::ops::Range {
//                                 start: center.x as i32,
//                                 end: coordinates[i].x as i32,
//                             };
//                             println!("2: range {:?}", range);
//                             for j in range {
//                                 let point = line.return_point_on_line(j as f32);
//                                 if circle.contains(point) == false {
//                                     endpoint = line.return_point_on_line(j as f32 - 0.5);
//                                     println!("\n2: endpoint point {:?}", endpoint);
//                                     line = shapes::Line::new(coordinates[i], endpoint);
//                                     graph.append(line.draw());
//                                     println!("nicht mehr");
//                                     break;

//                                 }else {
//                                     println!("noch drin");

//                                 };
//                             }

//                         }

//                     }

//                 }
//             }

//             graph

//         }

// }
