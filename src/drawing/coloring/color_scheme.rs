use std::str::FromStr;

use crate::utils::error::{self, BottomError};

#[derive(Debug)]
pub enum ColorScheme {
    Default,
    DefaultLight,
    Gruvbox,
    GruvboxLight,
    Nord,
    NordLight,
    Custom,
}

impl FromStr for ColorScheme {
    type Err = BottomError;

    fn from_str(s: &str) -> error::Result<Self> {
        let lower_case = s.to_lowercase();
        match lower_case.as_str() {
            "default" => Ok(ColorScheme::Default),
            "default-light" => Ok(ColorScheme::DefaultLight),
            "gruvbox" => Ok(ColorScheme::Gruvbox),
            "gruvbox-light" => Ok(ColorScheme::GruvboxLight),
            "nord" => Ok(ColorScheme::Nord),
            "nord-light" => Ok(ColorScheme::NordLight),
            _ => Err(BottomError::ConfigError(format!(
                "\"{}\" is an invalid built-in color scheme.",
                s
            ))),
        }
    }
}
