#![allow(non_snake_case)]
use crate::core::{
    constant::file_operations::{
        file_ai_features, file_basic_operations, file_informations, file_security, file_sharing,
        file_version,
    },
    models::operation_section::OperationSection,
};

pub fn FileOperations() -> Vec<OperationSection> {
    vec![
        OperationSection {
            title: "File Basic Operations",
            section_class: "file-basic-operations",
            item_class: "file-basic-operation-items",
            icon_prefix: "foi",
            items: file_basic_operations(),
        },
        OperationSection {
            title: "File Information",
            section_class: "file-information",
            item_class: "file-information-items",
            icon_prefix: "fii",
            items: file_informations(),
        },
        OperationSection {
            title: "File Sharing",
            section_class: "file-sharing",
            item_class: "file-sharing-items",
            icon_prefix: "fsi",
            items: file_sharing(),
        },
        OperationSection {
            title: "File Version",
            section_class: "file-version",
            item_class: "file-version-items",
            icon_prefix: "fvi",
            items: file_version(),
        },
        OperationSection {
            title: "File Security",
            section_class: "file-security",
            item_class: "file-security-items",
            icon_prefix: "fsi",
            items: file_security(),
        },
        OperationSection {
            title: "File AI Features",
            section_class: "file-ai-features",
            item_class: "file-ai-features-items",
            icon_prefix: "fai",
            items: file_ai_features(),
        },
    ]
}
