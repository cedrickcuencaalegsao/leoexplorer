#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeveloperType {
    // Source Code
    Rust,
    C,
    Cpp,
    Header,
    CSharp,
    Java,
    Kotlin,
    Swift,
    Go,
    Python,
    Ruby,
    Php,
    JavaScript,
    TypeScript,
    Jsx,
    Tsx,
    Dart,
    Lua,
    Perl,
    R,
    Scala,
    Haskell,
    Zig,

    // Web
    Html,
    Css,
    Scss,
    Sass,
    Less,

    // Config
    Json,
    Toml,
    Yaml,
    Xml,
    Env,
    Ini,
    Conf,

    // Build
    CargoToml,
    CargoLock,
    PackageJson,
    PackageLockJson,
    BunLock,
    YarnLock,
    PnpmLock,
    Makefile,
    CMake,
    Dockerfile,
    GitIgnore,
    GitAttributes,
    EditorConfig,

    // Database
    Sql,

    // Misc
    Log,
}
