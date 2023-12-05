
use voronator::VoronoiDiagram as VoiDi;
use voronator::delaunator::Point as VoiPoint;
use rand::prelude::*;
use rand::distributions::Uniform;

use crate::resources::layout;
use crate::resources::shapes::line::Line;
use crate::resources::shapes::point::Point;
pub struct Cell (pub Vec<Line>);
pub struct VoronoiDiagram {
    pub centers: Vec<(f64, f64)>,
    pub cells: Vec<Cell>,
    
}

impl VoronoiDiagram {
    pub fn new_uniform(layout: &layout::Grid, number_of_centers: i32) -> Self {
        
            let mut rng = rand::thread_rng();
            let height = Uniform::new(0., layout.height as f64);
            let width = Uniform::new(0., layout.width as f64);
            let centers: Vec<(f64, f64)> = (0..number_of_centers)
                .map(|_| (rng.sample(&width), rng.sample(&height)))
                .collect();
        
            let diagram = VoiDi::<VoiPoint>::from_tuple(&(0., 0.), &(layout.width as f64, layout.height as f64), &centers).unwrap();
        
            for cell in diagram.cells() {
                let poly: Vec<(f32, f32)> = cell
                    .points()
                    .into_iter()
                    .map(|x| (x.x as f32, x.y as f32))
                    .collect();   
            }

            let mut cells = Vec::new();
            for polygon in diagram.cells() {
                let mut cell = Vec::new();
        
                let points = polygon.points();
        
                for i in 0..points.len()-1 {
                    let line = Line::new(
                        Point::new(points[i].x as f32, points[i].y as f32),
                        Point::new(points[i+1].x as f32,points[i+1].y as f32)
                    );

                    cell.push(line);
                }
                // the closing line
                let line = Line::new(
                    Point::new(points[points.len()-1].x as f32, points[points.len()-1].y as f32),
                    Point::new(points[0].x as f32,points[0].y as f32)
                );
                cell.push(line);
                cells.push(Cell(cell));
                
            }
            let diagram = VoronoiDiagram {
                centers: centers,
                cells: cells,
            };
            diagram
        }


        pub fn new_from_points(layout: &layout::Grid, centers: Vec<(f64,f64)>) -> Self {
        
            let mut rng = rand::thread_rng();
            let height = Uniform::new(0., layout.height as f64);
            let width = Uniform::new(0., layout.width as f64);
        
            let diagram = VoiDi::<VoiPoint>::from_tuple(&(0., 0.), &(layout.width as f64, layout.height as f64), &centers).unwrap();

        
            for cell in diagram.cells() {
                let poly: Vec<(f32, f32)> = cell
                    .points()
                    .into_iter()
                    .map(|x| (x.x as f32, x.y as f32))
                    .collect();   
            }

            let mut cells = Vec::new();
            for polygon in diagram.cells() {
                let mut cell = Vec::new();
        
                let points = polygon.points();
        
                for i in 0..points.len()-1 {
                    let line = Line::new(
                        Point::new(points[i].x as f32, points[i].y as f32),
                        Point::new(points[i+1].x as f32,points[i+1].y as f32)
                    );

                    cell.push(line);
                }
                // the closing line
                let line = Line::new(
                    Point::new(points[points.len()-1].x as f32, points[points.len()-1].y as f32),
                    Point::new(points[0].x as f32,points[0].y as f32)
                );
                cell.push(line);
                cells.push(Cell(cell));
                
            }
            let diagram = VoronoiDiagram {
                centers: centers,
                cells: cells,
            };
            diagram
        }
}

// helpers

fn point_to_point(voi_point: (f64, f64)) -> Point {
    Point::new(
        voi_point.0 as f32,
        voi_point.1 as f32,
    )

}