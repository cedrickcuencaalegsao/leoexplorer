use crate::shared::constant::constant::{BORDER_RADIUS, FONT_SIZE, PADDING, PRIMARY_COLOR};

#[allow(dead_code)]
pub fn main_style() -> String {
    format! {
        "
        @font-face {{
            font-family: 'GoogleSans';
            src: url('/assets/fonts/GoogleSans-Regular.ttf') format('truetype');
            font-weight: 400;
            font-style: normal;
        }}
        @font-face {{
            font-family: 'GoogleSans';
            src: url('/assets/fonts/GoogleSans-Medium.ttf') format('truetype');
            font-weight: 500;
            font-style: normal;
        }}
        @font-face {{
            font-family: 'GoogleSans';
            src: url('/assets/fonts/GoogleSans-SemiBold.ttf') format('truetype');
            font-weight: 600;
            font-style: normal;
        }}
        @font-face {{
            font-family: 'GoogleSans';
            src: url('/assets/fonts/GoogleSans-Bold.ttf') format('truetype');
            font-weight: 700;
            font-style: normal;
        }}
        body {{
            height: 100vh;
            margin: 0;
            padding: 0;
            font-family: 'GoogleSans', Arial, sans-serif;
        }}
        main {{
            height: 100vh;
            margin: 0;
            padding: 0;
        }}
        .app-container {{
            display: flex;
            height: 100vh;
            background-color: #EAEAEA;
        }}
        .side-panel-container {{
            width: 15%;
            min-width: 200px;
            background-color: #ffffff;
            overflow: hidden;
            display: flex;
            flex-direction: column;
            padding: 10px;
        }}
        .main-panel-container {{
            flex: 1;
            border: 1px solid red;
        }}
        .preview-panel-container {{
            flex: 1;
            border: 1px solid green;
        }}
        .dynamic-sidebar-container {{
            flex: 1;
            border: 1px solid blue;
        }}
        "

    }
}

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

#[allow(dead_code)]
pub fn side_panel_style() -> String {
    format! {
        ".side-panel {{
            height: 100%;
            width: 100%;
            overflow-y: auto;
            height: 100%;
        }}"
    }
}

#[allow(dead_code)]
pub fn sp_menu_style() -> String {
    format! {
        ".sp-menu{{
            font-family: 'GoogleSans', Arial, sans-serif;
            font-weight: 700;
            font-size: 12px;
        }}"
    }
}
