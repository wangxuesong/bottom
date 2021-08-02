use super::BottomWidget;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Disk;
impl<B: tui::backend::Backend> BottomWidget<B> for Disk {
    fn create_element<'a>(&self) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}
