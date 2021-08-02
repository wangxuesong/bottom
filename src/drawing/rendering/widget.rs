use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    Frame,
};

use super::Node;

pub trait Widget<B: Backend> {
    /// How the [`Widget`] should be drawn, given a [`Node`] for its layout..
    fn draw(&mut self, ctx: &mut Frame<'_, B>, node: &'_ Node);

    /// How the [`Widget`] should be laid out given boundaries.
    fn layout(&self, bounds: Rect) -> Node;

    /// Returns the width of the [`Widget`]
    fn width(&self) -> Constraint;

    /// Returns the height of the [`Widget`]
    fn height(&self) -> Constraint;
}
