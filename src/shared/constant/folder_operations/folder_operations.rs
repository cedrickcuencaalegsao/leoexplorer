use crate::shared::{constant::folder_operations::*, models::operation_section::OperationSection};

pub fn folder_operations() -> Vec<OperationSection> {
    vec![
        OperationSection {
            title: "Basic Operations",
            section_class: "basic-operations",
            item_class: "basic-operation-items",
            icon_prefix: "boi",
            items: folder_basic_operations(),
        },
        OperationSection {
            title: "Folder Information",
            section_class: "folder-information",
            item_class: "folder-information-items",
            icon_prefix: "info",
            items: folder_information(),
        },
        OperationSection {
            title: "Folder Management Operations",
            section_class: "folder-management-operations",
            item_class: "folder-management-operation-items",
            icon_prefix: "fm",
            items: folder_management(),
        },
        OperationSection {
            title: "Navigation Operations",
            section_class: "navigation-operations",
            item_class: "navigation-operation-items",
            icon_prefix: "nav",
            items: navigation(),
        },
        OperationSection {
            title: "Folder AI Features",
            section_class: "folder-ai-features",
            item_class: "folder-ai-feature-items",
            icon_prefix: "ai",
            items: folder_ai_features(),
        },
        OperationSection {
            title: "Folder Advanced Options",
            section_class: "folder-advanced-options",
            item_class: "folder-advanced-option-items",
            icon_prefix: "adv",
            items: folder_advance_options(),
        },
        OperationSection {
            title: "Organization Operations",
            section_class: "organization-operations",
            item_class: "organization-operation-items",
            icon_prefix: "org",
            items: get_organization_operations(),
        },
    ]
}
