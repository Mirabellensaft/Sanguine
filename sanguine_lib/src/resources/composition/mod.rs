use super::shapes::{point::Point, line::Line};


/// This module only provides a composition trait and some enums to help describe compositional
/// properties. How they are rendered depends highly on the
/// individual project. It's probably helpful to provide a code template for this.


pub trait Composition {
    fn filled(&mut self, density_var: &Density);
    fn add_random_center(&mut self, amount: usize); 
    fn add_random_low(&mut self, amount: usize);
    fn connect_centers(&mut self);
    fn add_center(&mut self, center: CompositionCenter);
    fn retro_composition(&mut self);
    fn direction_of_contact(&mut self, row: usize, col: usize) -> Vec<bool>; 
}

// #[derive(Copy, Clone, Debug, PartialEq)]
// pub enum Density {
//     Transition(Direction),
//     ThreeWay(Direction),
//     Corner(Direction),
//     Edge(Direction),
//     Empty,
//     Low,
//     Mid,
//     High,
//     Focus,
// }

#[derive(Clone, Debug, PartialEq)]
pub enum Density {
    Transition(Direction),
    ThreeWay(Direction),
    Corner(Direction),
    Edge(Direction),
    Empty,
    Low,
    Mid,
    High,
    Focus,
}

/// Direction variants describe from where to where something is going in a field.
/// There is currently no 'None' option, a workouround is to just not read out a
/// set direction when rendering the Density variant that has a direction.
///
/// I omit an exact defintion of every single variant, as the interpretation will
/// vary from art work to art work.
#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    LeftRight,
    UpDown,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    Up,
    Down,
    Left,
    Right,
    Lines(Vec<Line>),
}

/// A possibility to set a center in the entire composition.
/// This is not properly implemented yet.
pub enum CompositionCenter {
    TopLeft,
    TopMid,
    TopRight,
    MidLeft,
    MidMid,
    MidRight,
    BottomLeft,
    BottomMid,
    BottomRight,
    Bottom,
    Top,
    Left,
    Right,
    VerticalCenter,
    HorizontalCenter,
}

