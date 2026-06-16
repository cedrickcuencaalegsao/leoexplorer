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
            position: relative;
            height: calc(100vh - 36px);
            margin: 0;
            padding: 0;
        }}
        .app-container {{
            display: flex;
            height: 100%;
            background-color: #EAEAEA;
        }}
        .side-panel-container {{
            width: 15%;
            min-width: 220px;
            background-color: #ffffff;
            overflow: hidden;
            display: flex;
            flex-direction: column;
            padding: 10px;
            color: #0a1931;
        }}
        .main-panel-container {{
            position: relative;
            flex: 1;
            min-width: 500px;
            background-color: #f6fafd;
        }}
        .tab-view{{
            position: absolute; inset: 0; display: none;
        }}
        .tab-view.active {{
            display: block;
        }}
        .preview-panel-container {{
            width: 100%;
            max-width: 300px;
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
pub fn title_bar_style() -> String {
    format! {
        ".title-bar {{
            display: flex;
            align-items: stretch;
            height: 36px;
            background: #dee1e6;
            user-select: none;
        }}
        .traffic-light-spacer {{
            flex: 0 0 78px;
        }}
        .tab-strip {{
            display: flex;
            align-items: center;
            flex: 0 1 auto;
            min-width: 0;
            overflow: hidden;
            padding-left: 4px;
        }}
        .title-bar-spacer {{
            flex: 1;
            min-width: 20px;
        }}
        .tab {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 8px;
            height: 34px;
            flex: 1 1 170px;
            min-width: 80px;
            width: 170px;
            max-width: 170px;
            margin: 6px 3px 0 3px;
            padding: 0 10px;
            cursor: pointer;
            border-radius: 8px 8px 0 0;
            background: transparent;
            border-right: 1px solid rgba(0,0,0,0.08);
            transition: background 0.15s ease;
        }}
        .tab:hover {{
            background: rgba(255, 255, 255, 0.4);
        }}
        .tab.active {{
            background: #fff;
            border-right: none;
        }}
        .tab span {{
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            min-width: 0;
            flex: 1;
            font-family: 'GoogleSans', Arial, sans-serif;
            font-size: 13px;
            font-weight: 500;
            color: #5f6368;
            transition: color 0.15s ease;
        }}
        .tab.active span {{
            color: #1a1a1a;
            font-weight: 600;
        }}
        .tab:hover span {{
            color: #1a1a1a;
        }}
        .tab-close {{
            flex: 0 0 auto;
            width: 20px;
            height: 20px;
            border-radius: 50%;
            background: transparent;
            border: none;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #5f6368;
            opacity: 0;
            transition: background 0.15s ease, opacity 0.1s ease;
        }}
        .tab:hover .tab-close, .tab.active .tab-close {{
            opacity: 1;
        }}
        .tab-close:hover {{
            background: rgba(0, 0, 0, 0.1);
            color: #1a1a1a;
        }}
        .tab-new {{
            flex: 0 0 auto;
            width: 28px;
            height: 28px;
            display: flex;
            align-items: center;
            justify-content: center;
            border: none;
            border-radius: 50%;
            background: transparent;
            cursor: pointer;
            font-size: 18px;
            color: #5f6368;
            margin: 6px;
            transition: background 0.15s ease;
        }}
        .tab-new:hover {{
            background: rgba(0, 0, 0, 0.1);
        }}
        .window-controls button {{
            color: #5f6368;
            transition: background 0.15s ease;
        }}
        .window-controls button:hover {{
            background: rgba(0,0,0,0.08);
        }}
        "
    }
}

#[allow(dead_code)]
pub fn windows_control_style() -> String {
    format! {
        ".window-controls {{
            display: flex;
        }}"
    }
}

#[allow(dead_code)]
pub fn linux_control_style() -> String {
    format! {
        ".window-controls {{
            display: flex;
        }}"
    }
}

#[allow(dead_code)]
pub fn macos_control_style() -> String {
    format! {
        ".window-controls {{
            display: flex;
            align-items: center;
            gap: 8px;
            width: 50px;
            padding: 0 12px;
        }}
        .window-controls button {{
            width: 12px;
            height: 12px;
            min-width: 12px;
            padding: 0;
            border: none;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            // -webkit-app-region: no-drag;
        }}
        .window-controls button svg {{
            opacity: 0;
            width: 8px;
            height: 8px;
            transition: opacity 0.1s ease;
        }}
        .window-controls:hover button svg {{
            opacity: 1;
        }}
        .traffic-close {{
            background: #ff5f57;
        }}
        .traffic-close svg {{
            color: #4d0000;
        }}
        .traffic-minimize {{
            background: #febc2e;
        }}
        .traffic-minimize svg {{
            color: #985700;
        }}
        .traffic-restore {{
            background: #28c840;
        }}
        .traffic-restore svg {{
            color: #006500;
        }}
        .window-controls button:active {{
            filter: brightness(0.85);
        }}"
    }
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
        .sp-header {{
            font-size: 14px;
            font-weight: bold;
            color: {GREY};
            margin-left: 10px;
        }}
        .library-wrapper, .sp-cloud-wrapper, .drive-list-container{{
            margin-left: 17px;
        }}
        "
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
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 6px 0;
            --drive-used-color: #332f6b;
            --drive-free-color: #e0e0e0;
        }}
        .sp-drive-icon {{
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            color: #1a1a1a;
        }}
        .sp-drive-info {{
            display: flex;
            flex-direction: column;
            gap: 4px;
            flex: 1;
            min-width: 0;
        }}
        .sp-drive-label {{
            font-size: 12px;
            font-weight: 700;
            font-style: italic;
            color: #1a1a1a;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }}
        .sp-drive-bar {{
            width: 100%;
            height: 5px;
            border-radius: 999px;
            background: var(--drive-free-color);
            overflow: hidden;
        }}
        .sp-drive-bar-used {{
            height: 100%;
            border-radius: 999px;
            background: var(--drive-used-color);
            transition: width 0.3s ease;
        }}
        .sp-drive-stats {{
            display: flex;
            align-items: center;
            gap: 10px;
            font-size: 9px;
            color: #999;
        }}
        .sp-drive-stat {{
            display: flex;
            align-items: center;
            gap: 4px;
        }}
        .sp-drive-dot {{
            width: 5px;
            height: 5px;
            border-radius: 50%;
            flex-shrink: 0;
        }}
        .sp-drive-dot-free {{
            background: #b3b3b3;
        }}
        .sp-drive-dot-used {{
            background: var(--drive-used-color);
        }}"
    }
}

pub fn sp_cloud_style() -> String {
    format!(
        ".sp-cloud {{
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
        .sp-cloud:hover {{
            background: rgba(0,0,0,0.06);
        }}
        .sp-cloud-children {{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
        }}
        .sp-cloud-icon{{
            display: flex;
            align-items: center;
        }}
        .sp-cloud-label p{{
            margin: 0;
            font-family: 'GoogleSans', Arial, sans-serif;
            font-weight: 600;
            font-size: 15px;
        }}
        .sp-cloud-open {{
            font-size: 15px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #2b2b2b;
            transition: transform 0.3s ease;
        }}"
    )
}
