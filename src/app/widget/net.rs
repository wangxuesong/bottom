use super::BottomWidget;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Net;
impl<B: tui::backend::Backend> BottomWidget<B> for Net {
    fn create_element<'a>(&self) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}
