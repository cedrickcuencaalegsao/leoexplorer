#![allow(non_snake_case)]
use crate::core::enums::kind::Kind;
use std::str::FromStr;

impl Kind {
    #[allow(dead_code)]
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            Kind::Image => &[
                "jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "tif", "heic",
                "heif", "raw", "cr2", "nef", "arw", "dng", "avif",
            ],
            Kind::Video => &[
                "mp4", "mov", "avi", "mkv", "wmv", "flv", "webm", "m4v", "mpeg", "mpg", "3gp", "ts",
            ],
            Kind::Audio => &[
                "mp3", "wav", "flac", "aac", "ogg", "m4a", "wma", "opus", "aiff", "aif",
            ],
            Kind::Document => &[
                "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "rtf",
                "txt", "csv", "md",
            ],
            Kind::Code => &[
                "rs", "py", "js", "ts", "jsx", "tsx", "html", "css", "json", "toml", "yaml", "yml",
                "sh", "bash", "zsh", "c", "cpp", "h", "hpp", "java", "kt", "swift", "go", "rb",
                "php", "lua", "dart", "sql",
            ],
            Kind::Archive => &[
                "zip", "tar", "gz", "bz2", "xz", "7z", "rar", "dmg", "iso", "tgz",
            ],
        }
    }
}

impl FromStr for Kind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "archive" => Ok(Kind::Archive),
            "audio" => Ok(Kind::Audio),
            "code" => Ok(Kind::Code),
            "document" => Ok(Kind::Document),
            "image" => Ok(Kind::Image),
            "video" => Ok(Kind::Video),
            _ => Err(()),
        }
    }
}
