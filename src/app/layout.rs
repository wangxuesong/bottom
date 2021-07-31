use std::fmt::Display;

use crate::error::{BottomError, Result};
use serde::{Deserialize, Serialize};
use typed_builder::*;

/// The [`BottomLayout`] is the widget layout derived from a config file.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BottomLayout {
    child: BottomLayoutChild,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum BottomLayoutChild {
    Widget {
        #[serde(rename = "type]")]
        widget_type: BottomWidgetType,
        percentage: u16,
    },
    Row {
        children: Vec<BottomLayoutChild>,
        percentage: u16,
    },
    Column {
        children: Vec<BottomLayoutChild>,
        percentage: u16,
    },
}

impl Default for BottomLayoutChild {
    fn default() -> Self {
        BottomLayoutChild::Widget {
            widget_type: BottomWidgetType::Empty,
            percentage: 100,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WidgetDirection {
    Left,
    Right,
    Up,
    Down,
}

impl WidgetDirection {
    pub fn is_opposite(&self, other_direction: &WidgetDirection) -> bool {
        match &self {
            WidgetDirection::Left => *other_direction == WidgetDirection::Right,
            WidgetDirection::Right => *other_direction == WidgetDirection::Left,
            WidgetDirection::Up => *other_direction == WidgetDirection::Down,
            WidgetDirection::Down => *other_direction == WidgetDirection::Up,
        }
    }
}

/// Represents a single widget.
#[derive(Debug, Default, Clone, TypedBuilder)]
pub struct BottomWidget {
    pub widget_type: BottomWidgetType,
    pub widget_id: u64,

    #[builder(default = 1)]
    pub width_ratio: u32,

    #[builder(default = None)]
    pub left_neighbour: Option<u64>,

    #[builder(default = None)]
    pub right_neighbour: Option<u64>,

    #[builder(default = None)]
    pub up_neighbour: Option<u64>,

    #[builder(default = None)]
    pub down_neighbour: Option<u64>,

    /// If set to true, the canvas will override any ratios.
    #[builder(default = false)]
    pub canvas_handle_width: bool,

    /// Whether we want this widget to take up all available room (and ignore any ratios).
    #[builder(default = false)]
    pub flex_grow: bool,

    /// The value is the direction to bounce, as well as the parent offset.
    #[builder(default = None)]
    pub parent_reflector: Option<(WidgetDirection, u64)>,

    /// Top left corner when drawn, for mouse click detection.  (x, y)
    #[builder(default = None)]
    pub top_left_corner: Option<(u16, u16)>,

    /// Bottom right corner when drawn, for mouse click detection.  (x, y)
    #[builder(default = None)]
    pub bottom_right_corner: Option<(u16, u16)>,
}

// TODO: Merge with widgets or separate out.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BottomWidgetType {
    Empty,
    Cpu,
    CpuLegend,
    Mem,
    Net,
    Proc,
    ProcSearch,
    ProcSort,
    Temp,
    Disk,
    BasicCpu,
    BasicMem,
    BasicNet,
    BasicTables,
    Battery,
}

impl BottomWidgetType {
    pub fn is_widget_table(&self) -> bool {
        use BottomWidgetType::*;
        matches!(self, Disk | Proc | ProcSort | Temp | CpuLegend)
    }

    pub fn is_widget_graph(&self) -> bool {
        use BottomWidgetType::*;
        matches!(self, Cpu | Net | Mem)
    }
}

impl Default for BottomWidgetType {
    fn default() -> Self {
        BottomWidgetType::Empty
    }
}

impl Display for BottomWidgetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BottomWidgetType::*;

        f.write_str(match self {
            Cpu => "CPU",
            Mem => "Memory",
            Net => "Network",
            Proc => "Processes",
            Temp => "Temperature",
            Disk => "Disks",
            Battery => "Battery",
            _ => "",
        })
    }
}

impl std::str::FromStr for BottomWidgetType {
    type Err = BottomError;

    fn from_str(s: &str) -> Result<Self> {
        let lower_case = s.to_lowercase();
        match lower_case.as_str() {
            "cpu" => Ok(BottomWidgetType::Cpu),
            "mem" | "memory" => Ok(BottomWidgetType::Mem),
            "net" | "network" => Ok(BottomWidgetType::Net),
            "proc" | "process" | "processes" => Ok(BottomWidgetType::Proc),
            "temp" | "temperature" => Ok(BottomWidgetType::Temp),
            "disk" => Ok(BottomWidgetType::Disk),
            "empty" => Ok(BottomWidgetType::Empty),
            "battery" | "batt" => Ok(BottomWidgetType::Battery),
            _ => Err(BottomError::ConfigError(format!(
                "\"{}\" is an invalid widget name.",
                s
            ))),
        }
    }
}

impl Serialize for BottomWidgetType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for BottomWidgetType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
