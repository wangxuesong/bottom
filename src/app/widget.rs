pub mod cpu;
pub use self::cpu::Cpu;

pub mod mem;
pub use self::mem::Mem;

pub mod net;
pub use self::net::Net;

pub mod proc;
pub use self::proc::Proc;

pub mod temp;
pub use self::temp::Temp;

pub mod disk;
pub use self::disk::Disk;

pub mod basic_cpu;
pub use self::basic_cpu::BasicCpu;

pub mod basic_mem;
pub use self::basic_mem::BasicMem;

pub mod basic_net;
pub use self::basic_net::BasicNet;

pub mod battery;
pub use self::battery::Battery;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tui::backend::Backend;

use crate::drawing::Element;
use crate::error::{BottomError, Result};

#[enum_dispatch(BottomWidget)]
#[derive(Debug, Clone)]
pub enum BottomWidgetType {
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

#[enum_dispatch]
pub trait BottomWidget<B: Backend> {
    fn create_element<'a>(&self) -> Element<'a, B>;
}

impl Display for BottomWidgetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BottomWidgetType::*;

        f.write_str(match self {
            Cpu(_) => "CPU",
            Mem(_) => "Memory",
            Net(_) => "Network",
            Proc(_) => "Processes",
            Temp(_) => "Temperature",
            Disk(_) => "Disks",
            Battery(_) => "Battery",
            _ => "",
        })
    }
}

impl std::str::FromStr for BottomWidgetType {
    type Err = BottomError;

    fn from_str(s: &str) -> Result<BottomWidgetType> {
        let lower_case = s.to_lowercase();
        match lower_case.as_str() {
            "cpu" => Ok(BottomWidgetType::Cpu(Cpu::default())),
            "mem" | "memory" => Ok(BottomWidgetType::Mem(Mem)),
            "net" | "network" => Ok(BottomWidgetType::Net(Net)),
            "proc" | "process" | "processes" => Ok(BottomWidgetType::Proc(Proc)),
            "temp" | "temperature" => Ok(BottomWidgetType::Temp(Temp)),
            "disk" => Ok(BottomWidgetType::Disk(Disk)),
            "battery" | "batt" => Ok(BottomWidgetType::Battery(Battery)),
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
