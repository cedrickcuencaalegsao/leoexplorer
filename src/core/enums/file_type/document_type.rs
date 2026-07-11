#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentType {
    Pdf,

    // Microsoft Office
    Word,
    WordDoc,
    WordDocx,

    Excel,
    ExcelXls,
    ExcelXlsx,

    PowerPoint,
    PowerPointPpt,
    PowerPointPptx,

    // OpenDocument
    Odt,
    Ods,
    Odp,

    // Text
    Txt,
    Markdown,
    RichText,
    Csv,
    Tsv,

    // Publishing
    Epub,
    Mobi,

    // Adobe
    Psd,
    Ai,
    Indd,

    // Documents
    Xml,
    Json,
    Yaml,
    Toml,
}
