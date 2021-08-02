use serde::{Deserialize, Serialize};

use super::widget::BottomWidgetType;

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
        BottomLayoutChild::Row {
            children: vec![],
            percentage: 100,
        }
    }
}
