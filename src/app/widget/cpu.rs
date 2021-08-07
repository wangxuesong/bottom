use super::BottomWidget;

#[derive(Debug, Clone, Default)]
pub struct Cpu {
    state: CpuState,
}

impl<B: tui::backend::Backend> BottomWidget<B> for Cpu {
    fn create_element<'a>(
        &'a self, app_state: &crate::app::AppState,
    ) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]

struct CpuState {
    current_display_time: u64,
    is_legend_hidden: bool,
    current_scroll_position: usize,
}
