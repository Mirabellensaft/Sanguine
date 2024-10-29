use crate::resources::shapes::point::Point;
use crate::resources::shapes::{circle::Circle, line::Line};
use lsystem::{LSystem, MapRules};
use std::f32::consts::PI;
use svg::node::element::Group;
use svg::Node;

/// This module implements applicable rules for L Systems described here https://paulbourke.net/fractals/lsys/.

/// This struct captures the current state of the Drawing. It is first defined with the starting properties.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct State {
    start: Point,
    length: f32,
    length_increment: f32,
    /// Angles in radians
    turning_angle: f32,
    turning_angle_increment: f32,

    current_direction: f32,
    /// default: false
    reverse: bool,
}

impl State {
    pub fn new(
        start: Point,
        length: f32,
        length_increment: f32,
        turning_angle: f32,
        turning_angle_increment: f32,
        current_direction: f32,
        reverse: bool,
    ) -> Self {
        let state = State {
            start: start,
            length: length,
            length_increment: length_increment,

            turning_angle: turning_angle,
            turning_angle_increment: turning_angle_increment,

            current_direction: current_direction,
            reverse: reverse,
        };
        state
    }
}

/// This Type serves as a stack for the different states, so it is possible to go back to a certain point.
pub struct SystemState(Vec<Vec<State>>);

impl SystemState {
    pub fn new(state: State) -> Self {
        let mut system_state: Vec<Vec<State>> = Vec::new();
        let vec = vec![state];
        system_state.push(vec);
        SystemState(system_state)
    }
    /// Gets the last added state.
    fn get_state(&self) -> State {
        let index = self.0.len() - 1;
        let state_keep = &self.0[index];
        let state = state_keep[state_keep.len() - 1];
        state
    }

    /// Updates the system with the new state
    fn update_state(&mut self, state: State) {
        let length = self.0.len() - 1;
        let state_keep = &mut self.0[length];
        state_keep.push(state);
    }

    /// Turns the string of chars that represent rules into a graphical representation.
    pub fn form_system(
        mut self,
        mut graph: Group,
        iteration: usize,

        mut lsystem: LSystem<char, MapRules<char>>,
    ) -> Group {
        let mut out: Vec<char> = Vec::new();
        for _i in 1..iteration {
            out = lsystem.next().unwrap();
        }

        for item in out {
            match item {
                'F' => {
                    let mut state = self.get_state();

                    let x = state.length * f32::cos(state.current_direction);
                    let y = state.length * f32::sin(state.current_direction);
                    let end = Point::new(state.start.x + x, state.start.y + y);
                    let line = Line::new(state.start, end);
                    graph.append(line.draw());
                    state.start = end;
                    self.update_state(state);
                }

                'f' => {
                    let mut state = self.get_state();
                    let x = state.length * f32::cos(state.current_direction);
                    let y = state.length * f32::sin(state.current_direction);
                    let end = Point::new(state.start.x + x, state.start.y + y);
                    state.start = end;
                    self.update_state(state);
                }

                '-' => {
                    let mut state = self.get_state();
                    if state.reverse == true {
                        state.current_direction = state.current_direction + state.turning_angle;
                    } else {
                        state.current_direction = state.current_direction - state.turning_angle;
                    }
                    self.update_state(state);
                }
                '+' => {
                    let mut state = self.get_state();
                    if state.reverse == true {
                        state.current_direction = state.current_direction - state.turning_angle;
                    } else {
                        state.current_direction = state.current_direction + state.turning_angle;
                    }
                    self.update_state(state);
                }

                '|' => {
                    let mut state = self.get_state();
                    state.current_direction = state.current_direction + PI;
                    self.update_state(state);
                }

                '[' => {
                    let state = self.get_state();
                    let vec = vec![state];
                    self.0.push(vec);
                }

                ']' => {
                    self.0.pop().unwrap();
                }
                '@' => {
                    // dot
                    let state = self.get_state();
                    let circle = Circle::new(state.start, 5.0);
                    graph.append(circle.draw());
                    self.update_state(state);
                }
                '>' => {
                    // increase line length
                    let mut state = self.get_state();
                    state.length = state.length + state.length_increment;
                    self.update_state(state);
                }
                '<' => {
                    // decrease line length;
                    let mut state = self.get_state();
                    state.length = state.length - state.length_increment;
                    self.update_state(state);
                }

                '&' => {
                    // swap meaning of + and -
                    let mut state = self.get_state();
                    if state.reverse == false {
                        state.reverse = true;
                    } else {
                        state.reverse = false;
                    }
                    self.update_state(state);
                }
                '(' => {
                    // decrease turning angle;
                    let mut state = self.get_state();
                    state.turning_angle = state.turning_angle - state.turning_angle_increment;
                    self.update_state(state);
                }
                ')' => {
                    // increase turning angle;
                    let mut state = self.get_state();
                    state.turning_angle = state.turning_angle - state.turning_angle_increment;
                    self.update_state(state);
                }
                _ => (),
            }
        }

        graph
    }
}
