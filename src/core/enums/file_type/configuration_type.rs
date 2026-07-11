#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigType {
    Json,
    Yaml,
    Toml,
    Xml,
    Ini,
    Conf,
    Env,
}
