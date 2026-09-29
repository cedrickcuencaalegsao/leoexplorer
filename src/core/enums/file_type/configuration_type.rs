use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigType {
    Json,
    Yaml,
    Toml,
    Xml,
    Ini,
    Conf,
    Env,
}
