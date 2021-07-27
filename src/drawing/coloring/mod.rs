pub mod canvas_colors;
pub use canvas_colors::CanvasColors;

pub mod color_scheme;
pub use color_scheme::ColorScheme;

use clap::ArgMatches;

use crate::{
    constants::{
        DEFAULT_LIGHT_MODE_COLOUR_PALETTE, GRUVBOX_COLOUR_PALETTE, GRUVBOX_LIGHT_COLOUR_PALETTE,
        NORD_COLOUR_PALETTE, NORD_LIGHT_COLOUR_PALETTE,
    },
    options::{get_color_scheme, Config},
};

pub fn generate_colour_scheme(
    matches: &ArgMatches<'static>, config: &Config,
) -> anyhow::Result<CanvasColors> {
    let colour_scheme = get_color_scheme(matches, config)?;

    let mut colors = CanvasColors::default();

    match colour_scheme {
        ColorScheme::Default => {
            // Don't have to do anything.
        }
        ColorScheme::DefaultLight => {
            colors.set_colors_from_config_colors(&*DEFAULT_LIGHT_MODE_COLOUR_PALETTE)?;
        }
        ColorScheme::Gruvbox => {
            colors.set_colors_from_config_colors(&*GRUVBOX_COLOUR_PALETTE)?;
        }
        ColorScheme::GruvboxLight => {
            colors.set_colors_from_config_colors(&*GRUVBOX_LIGHT_COLOUR_PALETTE)?;
        }
        ColorScheme::Nord => {
            colors.set_colors_from_config_colors(&*NORD_COLOUR_PALETTE)?;
        }
        ColorScheme::NordLight => {
            colors.set_colors_from_config_colors(&*NORD_LIGHT_COLOUR_PALETTE)?;
        }
        ColorScheme::Custom => {
            if let Some(config_colors) = &config.colors {
                colors.set_colors_from_config_colors(config_colors)?;
            }
        }
    }

    Ok(colors)
}
