use std::fmt::Display;

use crate::error::{BottomError, Result};
use serde::{Deserialize, Serialize};

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

// TODO: Merge with widgets or separate out.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BottomWidgetType {
    Empty,
    Cpu,
    Mem,
    Net,
    Proc,
    Temp,
    Disk,
    BasicCpu,
    BasicMem,
    BasicNet,
    Battery,
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
