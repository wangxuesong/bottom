use tui::{
    backend::Backend, layout::Constraint, style::Style as TuiStyle, text::Text, widgets::Paragraph,
};

use crate::drawing::{Element, Node, Widget};

/// The [`State`] of a [`Button`] mostly represents whether it is pressed or not.
#[derive(Default)]
pub struct State {
    is_pressed: bool,
}

/// The [`Style`] of a [`Button`] determines how it looks.
#[derive(Default)]
pub struct Style {
    not_pressed: TuiStyle,
    pressed: TuiStyle,
}

/// A [`Button`] is just a simple text element that supports "press" states.
pub struct Button<'a> {
    text: &'a str,
    state: &'a State,
    width: Constraint,
    height: Constraint,
    style: Style,
}

impl<'a> Button<'a> {
    /// Creates a new [`Button`].
    pub fn new(text: &'a str, state: &'a State) -> Self {
        Self {
            text,
            state,
            width: Constraint::Min(0),
            height: Constraint::Min(0),
            style: Style::default(),
        }
    }

    /// Sets the style of the [`Button`].
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the width of the [`Button`].

    pub fn width(mut self, width: Constraint) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Button`].

    pub fn height(mut self, height: Constraint) -> Self {
        self.height = height;
        self
    }
}

impl<'a, B> Widget<B> for Button<'a>
where
    B: Backend,
{
    fn draw(&mut self, ctx: &mut tui::Frame<'_, B>, node: &'_ crate::drawing::Node) {
        let style = if self.state.is_pressed {
            self.style.pressed
        } else {
            self.style.not_pressed
        };

        ctx.render_widget(
            Paragraph::new(Text::styled(self.text, style)),
            node.bounds(),
        );
    }

    fn layout(&self, bounds: tui::layout::Rect) -> crate::drawing::Node {
        Node::new(bounds, vec![])
    }

    fn width(&self) -> tui::layout::Constraint {
        self.width
    }

    fn height(&self) -> tui::layout::Constraint {
        self.height
    }
}

impl<'a, B: Backend> From<Button<'a>> for Element<'a, B> {
    fn from(button: Button<'a>) -> Self {
        Element::new(Box::new(button))
    }
}
