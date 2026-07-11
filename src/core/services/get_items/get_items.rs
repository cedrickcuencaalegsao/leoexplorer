#![allow(non_snake_case)]
use crate::core::{
    enums::{file_type::*, item_type::ItemType, view_mode::ViewMode},
    models::items::Items,
};

pub fn GetItems(view_mode: ViewMode) -> Vec<Items> {
    let items: Vec<Items> = vec![
        Items {
            name: "Folder 1".to_string(),
            item_type: ItemType::Folder,
            date_created: "DD/MM/YYYY".to_string(),
            date_modified: "DD/MM/YYYY".to_string(),
            flag: None,
            path: "~/Desktop/Folder 1".to_string(),
            is_dir: true,
            view_mode,
        },
        Items {
            name: "Folder 2".to_string(),
            item_type: ItemType::Folder,
            date_created: "DD/MM/YYYY".to_string(),
            date_modified: "DD/MM/YYYY".to_string(),
            flag: Some("LOCKED".to_string()),
            path: "~/Desktop/Folder 2".to_string(),
            is_dir: true,
            view_mode,
        },
        Items {
            name: "Folder 3 looooooooooooongggggggggg naaaaaaaaammmmmmeeeeeee".to_string(),
            item_type: ItemType::Folder,
            date_created: "DD/MM/YYYY".to_string(),
            date_modified: "DD/MM/YYYY".to_string(),
            flag: Some("HIDDEN".to_string()),
            path: "~/Desktop/Folder 3 AHAHAHAHAAHAHAHAHAHAHAHAHAHAAHAHAHA".to_string(),
            is_dir: true,
            view_mode,
        },
        Items {
            name: "File name".to_string(),
            item_type: ItemType::File(FileType::Unknown),
            date_created: "DD/MM/YYYY".to_string(),
            date_modified: "DD/MM/YYYY".to_string(),
            flag: None,
            path: "~/Desktop/file name wAHAHAAHAHAHAAHAHAHAHAHAHAHAAH".to_string(),
            is_dir: false,
            view_mode,
        },
        Items {
            name: "file name".to_string(),
            item_type: ItemType::File(FileType::Document(DocumentType::Json)),
            date_created: "DD/MM/YYYY".to_string(),
            date_modified: "DD/MM/YYYY".to_string(),
            flag: None,
            path: "~/Desktop/file name".to_string(),
            is_dir: false,
            view_mode,
        },
    ];

    return items;
}
