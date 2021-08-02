use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    Frame,
};

use crate::drawing::{Border, Element, Node, Padding, Widget};

/// A [`Col`] widget displays its children in a one-dimensional array along a vertical axis.
pub struct Col<'a, B: Backend> {
    children: Vec<Element<'a, B>>,
    padding: Padding,
    border: Border,
    width: Constraint,
    height: Constraint,
}

impl<'a, B: Backend> Col<'a, B> {
    /// Creates a new [`Col`] widget with no children.
    pub fn new() -> Self {
        Col::new_with_children(vec![])
    }

    /// Creates a new [`Col`] widget with the given [`Element`]s.
    pub fn new_with_children(children: Vec<Element<'a, B>>) -> Self {
        Self {
            children: children.into_iter().collect(),
            padding: Padding::Disabled,
            border: Border::Disabled,
            width: Constraint::Min(0),
            height: Constraint::Min(0),
        }
    }

    /// Sets the padding.
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the border.  Note that a border on a side takes up a single unit in terms of width/height.
    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Pushes a new element onto the [`Col`].
    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Element<'a, B>>,
    {
        self.children.push(child.into());
        self
    }

    /// Sets the width of the [`Col`].
    pub fn width(mut self, width: Constraint) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Col`].
    pub fn height(mut self, height: Constraint) -> Self {
        self.height = height;
        self
    }
}

impl<'a, B: Backend> Widget<B> for Col<'a, B> {
    fn draw(&mut self, ctx: &mut Frame<'_, B>, node: &'_ Node) {
        self.children
            .iter_mut()
            .zip(node.children())
            .for_each(|(child, child_node)| {
                child.draw(ctx, child_node);
            });
    }

    fn layout(&self, bounds: Rect) -> Node {
        use tui::layout::{Direction, Layout};

        let desired_constraints: Vec<Constraint> =
            self.children.iter().map(Element::height).collect();

        let constrained_bounds = Layout::default()
            .constraints(desired_constraints)
            .direction(Direction::Vertical)
            .split(bounds);

        Node::new(
            bounds,
            self.children
                .iter()
                .zip(constrained_bounds)
                .map(|(child, bound)| child.layout(bound))
                .collect(),
        )
    }

    fn width(&self) -> Constraint {
        self.width
    }

    fn height(&self) -> Constraint {
        self.height
    }
}

impl<'a, B: 'a + Backend> From<Col<'a, B>> for Element<'a, B> {
    fn from(view: Col<'a, B>) -> Self {
        Element::new(Box::new(view))
    }
}
