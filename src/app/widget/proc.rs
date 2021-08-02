use super::BottomWidget;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Proc;
impl<B: tui::backend::Backend> BottomWidget<B> for Proc {
    fn create_element<'a>(&self) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}
