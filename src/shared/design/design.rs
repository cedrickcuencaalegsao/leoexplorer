use crate::shared::constant::constant::{
    BORDER_RADIUS, DARK_BLUE, FONT_SIZE, GREY, PADDING, PRIMARY_COLOR,
};

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
            min-width: 180px;
            background-color: #ffffff;
            overflow: hidden;
            display: flex;
            flex-direction: column;
            padding: 10px;
            color: #0a1931;
        }}
        .main-panel-container {{
            flex: 1;
            min-width: 300px;
            background-color: #f6fafd;
        }}
        .preview-panel-container {{
            width: 100%;
            max-width: 500px;
            background-color: #b3cfe5;
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
            width: 100%;
            height: 100%;
            overflow-y: auto;
            padding-right: 10px;

            /* Firefox */
            scrollbar-width: thin;
            scrollbar-color: rgba(0,0,0,0.2) transparent;
        }}
        /* Chrome, Edge, Safari */
        .side-panel::-webkit-scrollbar {{
            width: 5px;
        }}

        .side-panel::-webkit-scrollbar-track {{
            background: transparent;
        }}

        .side-panel::-webkit-scrollbar-thumb {{
            background: rgba(0,0,0,0.18);
            border-radius: 999px;
        }}

        .side-panel::-webkit-scrollbar-thumb:hover {{
            background: rgba(0,0,0,0.3);
        }}
        .app-name-and-icon-container{{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
            margin-bottom: 10px;
        }}
        .app-icon img{{
            width: 35px;
            height: 35px;
        }}
        .app-name {{
            font-size: {FONT_SIZE};
            color: {DARK_BLUE};
            font-weight: 700;
            font-family: Google Sans, sans-serif;
            font-style: italic;
            font-size: 18px;
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
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
            justify-content: space-between;
            margin: 0px 0;
            padding: 7px;
            border-radius: 7px;
            cursor: pointer;
            color: {GREY};
            transition: all 0.3s ease;
        }}
        .sp-menu-open {{
            color: {DARK_BLUE};
        }}
        .sp-menu:hover {{
            background: rgba(0,0,0,0.06);
        }}
        .sp-menu-wrapper {{
            display: flex;
            flex-direction: column;
            width: 100%;
        }}
        .sp-menu-dropdown {{
            display: grid;
            grid-template-rows: 0fr;
            padding-left: 7px;
            gap: 2px;
            margin-left: 16px;
            border-left: 2px solid {GREY};
            transition: grid-template-rows 0.3s ease;
        }}
        .sp-menu-dropdown.open {{
            grid-template-rows: 1fr;
        }}
        .sp-menu-dropdown > * {{
            overflow: hidden;
        }}
        .sp-menu-children {{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
        }}
        .sp-icon{{
            display: flex;
            align-items: center;
        }}
        .sp-label p{{
            margin: 0;
            font-family: 'GoogleSans', Arial, sans-serif;
            font-weight: 600;
            font-size: 15px;
        }}
        .sp-chevron{{
            font-size: 15px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #2b2b2b;
            transition: transform 0.3s ease;
        }}
        .sp-chevron-up {{
            transform: rotate(180deg);
        }}
        "
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
            font-family: 'GoogleSans', Arial, sans-serif;
            font-weight: 700;
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
