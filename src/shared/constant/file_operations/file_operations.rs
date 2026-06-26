#![allow(non_snake_case)]
use crate::shared::{
    constant::file_operations::file_basic_operations, models::operation_section::OperationSection,
};

pub fn FileOperations() -> Vec<OperationSection> {
    vec![OperationSection {
        title: "",
        section_class: "",
        item_class: "",
        icon_prefix: "",
        items: file_basic_operations(),
    }]
}
