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
        @font-face {{
            font-family: 'GoogleSans';
            src: url('/assets/fonts/GoogleSans-BoldItalic.ttf') format('truetype');
            font-weight: 700;
            font-style: italic;
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
            min-width: 170px;
            background-color: #ffffff;
            overflow: hidden;
            display: flex;
            flex-direction: column;
            padding: 10px;
        }}
        .main-panel-container {{
            flex: 1;
            min-width: 300px;
        }}
        .preview-panel-container {{
            width: 100%;
            max-width: 500px;
            background-color: #fafaff;
        }}
        .dynamic-sidebar-container {{
            width: 100%;
            max-width: 50px;
            background-color: #ffffff;
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
        }}
        .drive-list-container {{
            margin-top: 30px;
        }}"
    }
}

#[allow(dead_code)]
pub fn sp_menu_style() -> String {
    format! {
        ".sp-menu {{
            margin: 10px 0;
            cursor: pointer;
        }}
        .sp-menu-children {{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap:8px;
        }}
        .sp-icon{{
            display: flex;
            align-items: center;
        }}
        .sp-label p{{
            margin: 0;
            font-family: 'GoogleSans', Arial, sans-serif;
            font-weight: 700;
            font-size: 12px;
        }}"
    }
}

pub fn sp_items_style() -> String {
    format!(
        ".sp-items {{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
            padding: 5px 7px;
            border-radius: 7px;
            cursor: pointer;
            transition: background 0.2s ease;
            font-family: 'GoogleSans', Arial, sans-serif;
        }}
        .sp-items:hover {{
            background: rgba(0,0,0,0.06);
        }}
        .sp-items-icon {{
            display: flex;
            align-items: center;
            color: #e8a020;  /* orange for folders */
        }}
        .sp-items-label p {{
            margin: 0;
            font-family: 'GoogleSans', Arial, sans-serif;
            font-weight: 500;
            font-size: 14px;
            color: #21222d;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            max-width: 130px;
        }}"
    )
}

#[allow(dead_code)]
pub fn sp_drive_style() -> String {
    format! {
        ".sp-drive {{
            margin: 10px 0;
            cursor: pointer;
        }}
        .sp-drive-children {{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap:8px;
        }}
        .sp-drive-icon {{
            font-size: 16px;
        }}
        .sp-drive-label {{
            font-size: 14px;
        }}"
    }
}
