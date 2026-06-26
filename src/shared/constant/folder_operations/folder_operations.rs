use crate::shared::{
    constant::folder_operations::*, models::folder_operation_section::FolderOperationSection,
};

pub fn folder_operations() -> Vec<FolderOperationSection> {
    vec![
        FolderOperationSection {
            title: "Basic Operations",
            section_class: "basic-operations",
            item_class: "basic-operation-items",
            icon_prefix: "boi",
            items: folder_basic_operations(),
        },
        FolderOperationSection {
            title: "Folder Information",
            section_class: "folder-information",
            item_class: "folder-information-items",
            icon_prefix: "info",
            items: folder_information(),
        },
        FolderOperationSection {
            title: "Folder Management Operations",
            section_class: "folder-management-operations",
            item_class: "folder-management-operation-items",
            icon_prefix: "fm",
            items: folder_management(),
        },
        FolderOperationSection {
            title: "Navigation Operations",
            section_class: "navigation-operations",
            item_class: "navigation-operation-items",
            icon_prefix: "nav",
            items: navigation(),
        },
        FolderOperationSection {
            title: "Folder AI Features",
            section_class: "folder-ai-features",
            item_class: "folder-ai-feature-items",
            icon_prefix: "ai",
            items: folder_ai_features(),
        },
        FolderOperationSection {
            title: "Folder Advanced Options",
            section_class: "folder-advanced-options",
            item_class: "folder-advanced-option-items",
            icon_prefix: "adv",
            items: folder_advance_options(),
        },
        FolderOperationSection {
            title: "Organization Operations",
            section_class: "organization-operations",
            item_class: "organization-operation-items",
            icon_prefix: "org",
            items: get_organization_operations(),
        },
    ]
}
