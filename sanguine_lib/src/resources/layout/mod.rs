#[allow(implied_bounds_entailment)]

use svg::{
    node::{
        self,
        element::{path::Data, Path},
    },
    Node,
};

use super::layout::{grid::{Grid, Field}, voronoi::{VoronoiDiagram, Cell}};

use super::shapes::point::Point;

pub mod grid;
pub mod voronoi;

/// This module contains types that describe the fundamental properties of the work.
///
/// Currently everything is grid based.

/// Format contains the works's properties and a container for the grid.

pub trait Layout {
    fn new(parameters: Parameters) -> Self where Self: Sized;
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_rows(&self) -> usize;
    fn get_columns(&self) -> usize;
    fn get_points(&self) -> Vec<Cell>;
    fn get_fields(&self) -> Vec<Vec<Field>>;
    fn get_grid(&self) -> Grid;
}

pub struct Parameters {
    pub height: i32, 
    pub width: i32, 
    pub margin: i32, 
    pub rows: usize, 
    pub columns: usize,
    pub layout_type: LayoutType
}
pub struct Work(pub Box <dyn Layout>);

pub enum VoronoiType {
    Custom(Vec<Point>),
    Uniform(i32),
}

pub enum LayoutType {
    GridBased(usize, usize),
    VoronoiBased(VoronoiType),

}

impl Work {
    pub fn new(parameters: Parameters) -> Self {
        match parameters.layout_type {
            LayoutType::GridBased(_,_) => {
                Work(Box::new(Grid::new(parameters)))
            },
            LayoutType::VoronoiBased(_) => {
                Work(Box::new(VoronoiDiagram::new(parameters)))
            },
        }
    }

        /// Adds a white background so png conversion is easier.
    pub fn background(&self) -> node::element::Group {
        let mut graph = node::element::Group::new();

        let data = node::element::path::Data::new()
            .move_to((0, 0))
            .line_to((0, self.0.get_height()))
            .line_to((self.0.get_width(), self.0.get_height()))
            .line_to((self.0.get_width(), 0))
            .close();

        let path = path(data);
        graph.append(path);
        graph
    }

}
/// Helper function for the background path.
fn path(data: Data) -> Path {
    let path = Path::new()
        .set("fill", "white")
        .set("stroke", "none")
        .set("stroke-width", 1)
        .set("d", data);

    path
}
    
/// Orientation type, not implemented for use.
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
}

