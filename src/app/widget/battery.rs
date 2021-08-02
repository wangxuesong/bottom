use super::BottomWidget;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Battery;
impl<B: tui::backend::Backend> BottomWidget<B> for Battery {
    fn create_element<'a>(&self) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}
