use crate::core::enums::{
    cloud_status::CloudStatus, explorer_state::ExplorerState, fs_attribute::FsAttribute,
    item_permissions::ItemPermissions,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Flag {
    Attribute(FsAttribute),
    ItemPermission(ItemPermissions),
    Explorer(ExplorerState),
    Cloud(CloudStatus),
}
