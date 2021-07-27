use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::drawing::{Element, Node, Widget};

/// Represents the state of a [`Carousel`].
pub struct State {
    /// This must never exceed the length of the elements.
    selected_index: usize,
}

pub struct NamedElement<'a, B>
where
    B: Backend,
{
    element: Element<'a, B>,
    name: &'a str,
}

/// A [`Carousel`] is a widget that shows only one of its children element at a time.
pub struct Carousel<'a, B>
where
    B: Backend,
{
    state: &'a mut State,
    children: Vec<NamedElement<'a, B>>,
    width: Constraint,
    height: Constraint,
}

impl<'a, B: Backend> Carousel<'a, B> {
    /// Creates a new [`Carousel`].
    pub fn new(state: &'a mut State, children: Vec<NamedElement<'a, B>>) -> Self {
        Self {
            state,
            children,
            width: Constraint::Min(0),
            height: Constraint::Min(0),
        }
    }

    /// Sets the width of the widget.
    pub fn width(mut self, width: Constraint) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the widget.
    pub fn height(mut self, height: Constraint) -> Self {
        self.height = height;
        self
    }

    /// This unsafe call is safe due to internal checks.
    fn get_prev_element(&self) -> &NamedElement<'a, B> {
        unsafe {
            self.children
                .get_unchecked(if self.state.selected_index > 0 {
                    self.state.selected_index - 1
                } else {
                    self.children.len() - 1
                })
        }
    }

    /// This unsafe call is safe due to internal checks.
    fn get_next_element(&self) -> &NamedElement<'a, B> {
        unsafe {
            self.children.get_unchecked({
                let next_index = self.state.selected_index + 1;
                if next_index == self.children.len() {
                    0
                } else {
                    next_index
                }
            })
        }
    }

    /// This unsafe call is only safe if we ensure the index is always within the length.  This should be done by only using
    /// [`set_selected_index`].
    fn get_current_element(&self) -> &NamedElement<'a, B> {
        unsafe { self.children.get_unchecked(self.state.selected_index) }
    }

    /// This unsafe call is only safe if we ensure the index is always within the length.  This should be done by only using
    /// [`set_selected_index`].
    fn get_mut_current_element(&mut self) -> &mut NamedElement<'a, B> {
        unsafe { self.children.get_unchecked_mut(self.state.selected_index) }
    }

    /// Sets the selected index.
    pub fn set_selected_index(&mut self, new_index: usize) {
        self.state.selected_index = new_index % self.children.len();
    }
}

impl<'a, B: Backend> Widget<B> for Carousel<'a, B> {
    fn draw(&mut self, ctx: &mut tui::Frame<'_, B>, node: &'_ crate::drawing::Node) {
        // Draw arrows
        let prev_element_name = self.get_prev_element().name;
        let next_element_name = self.get_next_element().name;

        let prev_arrow_text = vec![
            Spans::default(),
            Spans::from(Span::raw(format!("◄ {}", prev_element_name))),
        ];

        let next_arrow_text = vec![
            Spans::default(),
            Spans::from(Span::raw(format!("{} ►", next_element_name))),
        ];

        // Now draw the rest of the current element...
        self.get_mut_current_element().element.draw(
            ctx,
            // This unsafe call is safe, since we always create a layout with 2 children.
            unsafe { node.children().get_unchecked(1) },
        )
    }

    fn layout(&self, bounds: tui::layout::Rect) -> crate::drawing::Node {
        let split_bounds = Layout::default()
            .constraints(vec![
                tui::layout::Constraint::Length(2),
                tui::layout::Constraint::Min(0),
            ])
            .direction(tui::layout::Direction::Vertical)
            .split(bounds);

        Node::new(
            bounds,
            vec![
                Node::new(split_bounds[0], vec![]),
                self.get_current_element().element.layout(split_bounds[1]),
            ],
        )
    }

    fn width(&self) -> tui::layout::Constraint {
        self.width
    }

    fn height(&self) -> tui::layout::Constraint {
        self.height
    }
}
