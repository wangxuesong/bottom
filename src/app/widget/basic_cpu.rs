use super::BottomWidget;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BasicCpu;
impl<B: tui::backend::Backend> BottomWidget<B> for BasicCpu {
    fn create_element<'a>(&self) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}
