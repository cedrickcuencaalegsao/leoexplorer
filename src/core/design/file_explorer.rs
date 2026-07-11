use crate::core::constant::constant::LIGHT_GREY;

pub fn file_explorer_style() -> String {
    format! {
        "
            .file-explorer-main-container{{
                padding: 0px 10px 0px 0px;
                height: 100%;
                width: 100%;
                background-color: {LIGHT_GREY};
            }}
            .item-container{{
                display: flex;
                flex-direction: column;
                margin: 10px 0 0 0;
            }}
            .items{{
                display: flex;
                flex-direction: column;
                gap: 10px;
            }}
        "
    }
}
