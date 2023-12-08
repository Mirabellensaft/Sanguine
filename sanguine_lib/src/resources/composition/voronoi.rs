use rand::distributions::Uniform;
use rand::prelude::*;
use voronator::delaunator::Point as VoiPoint;
use voronator::VoronoiDiagram as VoiDi;

use crate::resources::{
    layout,
    shapes::{line::Line, point::Point},
};

#[derive()]
pub struct Cell {
    pub border_lines: Vec<Line>,
    pub center: Point,
}
pub struct VoronoiDiagram {
    pub centers: Vec<(f64, f64)>,
    pub cells: Vec<Cell>,
}

pub enum VoronoiType {
    Custom(Vec<Point>),
    Uniform(i32),
}

impl VoronoiDiagram {
    pub fn new(layout: &layout::Grid, diagram_type: VoronoiType) -> Self {
        let mut rng = rand::thread_rng();
        let height = Uniform::new(0., layout.height as f64);
        let width = Uniform::new(0., layout.width as f64);

        let mut centers = Vec::new();

        match diagram_type {
            VoronoiType::Custom(points) => {
                for point in points {
                    centers.push((point.x as f64, point.y as f64))
                }
            }
            VoronoiType::Uniform(number_of_centers) => {
                centers = (0..number_of_centers)
                    .map(|_| (rng.sample(&width), rng.sample(&height)))
                    .collect();
            }
        }

        let diagram = VoiDi::<VoiPoint>::from_tuple(
            &(0., 0.),
            &(layout.width as f64, layout.height as f64),
            &centers,
        )
        .unwrap();

        for cell in diagram.cells() {
            let _poly: Vec<(f32, f32)> = cell
                .points()
                .into_iter()
                .map(|x| (x.x as f32, x.y as f32))
                .collect();
        }

        let mut cells = Vec::new();

        for polygon in diagram.cells() {
            let mut cell_border_lines = Vec::new();

            let points = polygon.points();

            for i in 0..points.len() - 1 {
                let line = Line::new(
                    Point::new(points[i].x as f32, points[i].y as f32),
                    Point::new(points[i + 1].x as f32, points[i + 1].y as f32),
                );

                cell_border_lines.push(line);
            }
            // the closing line
            let line = Line::new(
                Point::new(
                    points[points.len() - 1].x as f32,
                    points[points.len() - 1].y as f32,
                ),
                Point::new(points[0].x as f32, points[0].y as f32),
            );
            cell_border_lines.push(line);

            // The centers for each cell are currently set to zero.
            cells.push(Cell {
                border_lines: cell_border_lines,
                center: Point::new(0.0, 0.0),
            });
        }
        let diagram = VoronoiDiagram {
            centers: centers,
            cells: cells,
        };
        diagram
    }
}

// helpers

// fn point_to_point(voi_point: (f64, f64)) -> Point {
//     Point::new(voi_point.0 as f32, voi_point.1 as f32)
// }
