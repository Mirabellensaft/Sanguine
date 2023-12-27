use crate::resources::{
    composition::Density,
    errors::Error,
    shapes::{line::Line, point::Point},
};
use rand::distributions::Uniform;
use rand::prelude::*;
use voronator::delaunator::Point as VoiPoint;
use voronator::VoronoiDiagram as VoiDi;

use super::{grid::Field, Layout, LayoutType, Parameters, VoronoiType};

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
    /// Vector that contains the voronoi centers.
    pub centers: Vec<(f64, f64)>,
    /// Vector that contains the voronoi cells.
    pub cells: Vec<Cell>,
}

impl Layout for VoronoiDiagram {
    /// generates a new voronoi layout
    fn new(parameters: Parameters) -> Result<Self, Error> {
        let mut rng = rand::thread_rng();
        let height = Uniform::new(0., parameters.height as f64);
        let width = Uniform::new(0., parameters.width as f64);
        let mut len_centers: usize = 0;

        let mut centers = Vec::new();

        match parameters.layout_type {
            LayoutType::VoronoiBased(voronoi_type) => match voronoi_type {
                VoronoiType::Custom(points) => {
                    for point in points {
                        len_centers += 1;
                        centers.push((point.x as f64, point.y as f64))
                    }
                }
                VoronoiType::Uniform(number_of_centers) => {
                    len_centers = number_of_centers as usize;
                    centers = (0..number_of_centers)
                        .map(|_| (rng.sample(&width), rng.sample(&height)))
                        .collect();
                }
            },
            LayoutType::GridBased(_, _) => return Err(Error::LayoutTypeError),
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

            // println!("Polygon Points {:?}", points);

            for i in 0..points.len() - 1 {
                let line = Line::new(
                    Point::new(points[i].x as f32, points[i].y as f32),
                    Point::new(points[i + 1].x as f32, points[i + 1].y as f32),
                );
                // println!("Line {:?}", line);
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
            // println!("Closing Line {:?}", line);
            cell_border_lines.push(line);

            // The centers for each cell are currently set to zero.
            cells.push(Cell {
                border_lines: cell_border_lines,
                center: Point::new(0.0, 0.0),
                density: Density::Empty,
            });
        }

        let mut work = VoronoiDiagram {
            height: parameters.height,
            width: parameters.width,
            margin: parameters.margin,
            centers: centers,
            cells: cells,
        };

        for i in 0..len_centers {
            work.cells[i].center = Point::new(work.centers[i].0 as f32, work.centers[i].1 as f32);
        }

        Ok(work)
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

    fn get_fields(&self) -> Vec<Vec<Field>> {
        unimplemented!()
    }

    // fn background(&self) -> svg::node::element::Group {
    //     todo!()
    // }
}

impl Cell {
    pub fn find_neighbors(&self, diagram: &VoronoiDiagram) -> Vec<(Point, Density, Line)> {
        let mut neighbors_centers = Vec::new();
        println!("self center: {:?}", self.center);
        for side in &self.border_lines {
            for cell in &diagram.get_points() {
                for other_side in &cell.border_lines {
                    if side.equal(*other_side) && cell.center != self.center {
                        println!(
                            "center: {:?}, density: {:?}, side: {:?}",
                            cell.center,
                            cell.get_density(),
                            *side,
                        );
                        neighbors_centers.push((cell.center, cell.get_density(), *side));
                    }
                }
            }
        }
        neighbors_centers
    }

    pub fn set_density(&mut self, density: Density) {
        self.density = density
    }

    pub fn get_density(&self) -> Density {
        let density = self.density.clone();
        density
    }
}
