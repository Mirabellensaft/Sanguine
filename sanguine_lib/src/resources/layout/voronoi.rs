use rand::distributions::Uniform;
use rand::prelude::*;
use voronator::delaunator::Point as VoiPoint;
use voronator::VoronoiDiagram as VoiDi;
use crate::resources::{shapes::{point::Point, line::Line}, composition::Density};

use super::{Parameters, Layout, LayoutType, VoronoiType};

#[derive(Clone, Debug)]
pub struct Cell {
    pub border_lines: Vec<Line>,
    pub center: Point,
    pub density: Density,
}

/// Measurement properties of one field.
/// Maybe width and height can go somewhere else, because its the same for every field?

#[derive(Debug, Clone)]
pub struct VoronoiDiagram {
    /// Work height in pixels
    pub height: i32,
    /// Work width in pixels
    pub width: i32,
    /// A margin, value is currently never applied anywhere.
    pub margin: i32,

    pub centers: Vec<(f64, f64)>,
    /// Vector that contains points for generating a voronoi diagram, can be empty.
    pub cells: Vec<Cell>,
}

impl Layout for VoronoiDiagram{
    /// generates a new voronoi layout, with an empty point vector.
    fn new(parameters: Parameters) -> Self {
        let mut rng = rand::thread_rng();
        let height = Uniform::new(0., parameters.height as f64);
        let width = Uniform::new(0., parameters.width as f64);

        let mut centers = Vec::new();

        match parameters.layout_type {
            
            LayoutType::VoronoiBased(voronoi_type) => {
                match voronoi_type {
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
            },
            _ => (),
        }

        let diagram = VoiDi::<VoiPoint>::from_tuple(
            &(0., 0.),
            &(parameters.width as f64, parameters.height as f64),
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
                density: Density::Empty,
            });
        }

        let work = VoronoiDiagram {
            height: parameters.height,
            width: parameters.width,
            margin: parameters.margin,
            centers: centers,
            cells: cells,
        };

        work
    }


    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    fn get_rows(&self) -> usize {
        unimplemented!()
    }

    fn get_columns(&self) -> usize {
        unimplemented!()
    }

    fn get_points(&self) -> Vec<Cell> {
        self.cells.clone()
    }

    fn get_fields(&self) -> Vec<Vec<super::grid::Field>> {
        unimplemented!()
    }

    fn get_grid(&self) -> super::grid::Grid {
        unimplemented!()
    }
}
