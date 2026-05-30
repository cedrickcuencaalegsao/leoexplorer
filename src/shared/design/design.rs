use crate::shared::constant::constant::{BORDER_RADIUS, FONT_SIZE, PADDING, PRIMARY_COLOR};

#[allow(dead_code)]
pub fn primary_button_style() -> String {
    format!(
        "background-color: {}; font-size: {}; border-radius: {}; padding: {};",
        PRIMARY_COLOR, FONT_SIZE, BORDER_RADIUS, PADDING
    )
}

#[allow(dead_code)]
pub fn secondary_button_style() -> String {
    format!(
        "background-color: transparent; border: 2px solid {}; font-size: {}; border-radius: {};",
        PRIMARY_COLOR, FONT_SIZE, BORDER_RADIUS
    )
}
