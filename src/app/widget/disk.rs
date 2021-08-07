use super::BottomWidget;

#[derive(Debug, Clone, Default)]
pub struct Disk;
impl<B: tui::backend::Backend> BottomWidget<B> for Disk {
    fn create_element<'a>(
        &'a self, app_state: &crate::app::AppState,
    ) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}
