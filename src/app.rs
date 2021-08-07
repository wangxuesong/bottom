pub mod data_farmer;
pub mod data_harvester;
pub mod event_handler;
pub mod filter;
pub mod layout;
mod process_killer;
pub mod query;
pub mod widget;
pub mod widget_states;

use typed_builder::*;

use data_farmer::*;
use data_harvester::{processes, temperature};
pub use filter::*;
pub use widget_states::*;

use crate::{data_conversion::DisplayableData, units::data_units::DataUnit, Pid};

use self::data_harvester::UsedWidgets;

const MAX_SEARCH_LENGTH: usize = 200;

#[derive(Debug, Clone)]
pub enum AxisScaling {
    Log,
    Linear,
}

/// AppConfigFields is meant to cover basic fields that would normally be set
/// by config files or launch options.
#[derive(Debug)]
pub struct AppConfigFields {
    pub update_rate_in_milliseconds: u64,
    pub temperature_type: temperature::TemperatureType,
    pub use_dot: bool,
    pub left_legend: bool,
    pub show_average_cpu: bool,
    pub use_current_cpu_total: bool,
    pub use_basic_mode: bool,
    pub default_time_value: u64,
    pub time_interval: u64,
    pub hide_time: bool,
    pub autohide_time: bool,
    pub use_old_network_legend: bool,
    pub table_gap: u16,
    pub disable_click: bool,
    pub no_write: bool,
    pub show_table_scroll_position: bool,
    pub is_advanced_kill: bool,
    // TODO: Remove these, move network details state-side.
    pub network_unit_type: DataUnit,
    pub network_scale_type: AxisScaling,
    pub network_use_binary_prefix: bool,
}

#[derive(TypedBuilder)]
pub struct AppState {
    #[builder(default, setter(skip))]
    pub dd_err: Option<String>,

    #[builder(default, setter(skip))]
    to_delete_process_list: Option<(String, Vec<Pid>)>,

    #[builder(default = false, setter(skip))]
    pub is_frozen: bool,

    #[builder(default, setter(skip))]
    pub canvas_data: DisplayableData,

    #[builder(default, setter(skip))]
    pub data_collection: DataCollection,

    #[builder(default, setter(skip))]
    pub delete_dialog_state: AppDeleteDialogState,

    #[builder(default, setter(skip))]
    pub help_dialog_state: AppHelpDialogState,

    #[builder(default = false, setter(skip))]
    pub is_expanded: bool,

    #[builder(default = false, setter(skip))]
    pub is_force_redraw: bool,

    #[builder(default = false, setter(skip))]
    pub is_determining_widget_boundary: bool,

    #[builder(default = false, setter(skip))]
    pub basic_mode_use_percent: bool,

    #[cfg(target_family = "unix")]
    #[builder(default, setter(skip))]
    pub user_table: processes::UserTable,

    pub cpu_states: CpuState,
    pub mem_states: MemState,
    pub net_states: NetState,
    pub proc_states: ProcState,
    pub temp_states: TempState,
    pub disk_states: DiskState,
    pub battery_states: BatteryState,
    pub basic_table_widget_state: Option<BasicTableWidgetState>,
    pub app_config_fields: AppConfigFields,
    pub used_widgets: UsedWidgets,
    pub filters: DataFilters,
}

impl AppState {
    pub fn reset(&mut self) {
        // Reset dialog state
        self.help_dialog_state.is_showing_help = false;
        self.delete_dialog_state.is_showing_dd = false;

        // Close all searches and reset it
        self.proc_states
            .widget_states
            .values_mut()
            .for_each(|state| {
                state.process_search_state.search_state.reset();
            });
        self.proc_states.force_update_all = true;

        // Clear current delete list
        self.to_delete_process_list = None;
        self.dd_err = None;

        // Unfreeze.
        self.is_frozen = false;

        // Reset zoom

        // Reset data
        self.data_collection.reset();
    }
}
