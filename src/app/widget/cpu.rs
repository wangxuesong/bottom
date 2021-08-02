use crate::drawing::cpu_widget::State;

use super::BottomWidget;

#[derive(Debug, Clone, Default)]
pub struct Cpu {
    widget_state: CpuState,
}

impl<B: tui::backend::Backend> BottomWidget<B> for Cpu {
    fn create_element<'a>(&self) -> crate::drawing::Element<'a, B> {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]

struct CpuState {
    draw_state: State,
}
