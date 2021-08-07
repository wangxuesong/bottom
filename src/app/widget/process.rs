use super::BottomWidget;

#[derive(Debug, Clone, Default)]
pub struct Proc {
    state: ProcState,
}

impl<B: tui::backend::Backend> BottomWidget<B> for Proc {
    fn create_element<'a>(
        &'a self, app_state: &crate::app::AppState,
    ) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProcState {}

impl ProcState {
    pub fn get_process_filter(&self) {}

    // TODO: Should these be here or part of the text widget?
    fn clear_search(&mut self) {}

    fn clear_previous_word(&mut self) {}
}
