use super::BottomWidget;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BasicMem;
impl<B: tui::backend::Backend> BottomWidget<B> for BasicMem {
    fn create_element<'a>(&self) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}
