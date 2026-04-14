use regex::Regex;
use std::sync::OnceLock;

// App Info
pub const APP_NAME: &str = "Code Comment Cleaning Tool";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_SEPARATOR_LEN: usize = 50;
pub const APP_SEPARATOR_CHAR: &str = "=";

// Output Messages
pub const MSG_PROCESSING_COMPLETED: &str = "\nProcessing completed!";
pub const MSG_DRY_RUN_SKIP: &str = "  [Dry run - file not modified]";
pub const MSG_BACKUP_PREFIX: &str = "  Backup:";
pub const MSG_OUTPUT_PREFIX: &str = "  Output:";
pub const MSG_PROCESSING_PREFIX: &str = "Processing:";
pub const MSG_SKIPPING_PREFIX: &str = "Skipping:";
pub const MSG_ERROR_PREFIX: &str = "Error:";
pub const MSG_PROCESSED_SUMMARY: &str = "Processed";
pub const MSG_SKIPPED_SUMMARY: &str = "skipped";
pub const MSG_FILES_SUFFIX: &str = "files";

// Error Messages
pub const ERR_PATH_NOT_EXIST: &str = "Path does not exist:";
pub const ERR_UNRECOGNIZED_LANG_CLI: &str = "Cannot recognize the specified programming language";
pub const ERR_UNRECOGNIZED_LANG_EXT: &str = "Cannot recognize programming language from file extension, please use -l parameter to specify manually";
pub const ERR_READ_FILE: &str = "Cannot read file:";
pub const ERR_CREATE_BACKUP: &str = "Cannot create backup file:";
pub const ERR_WRITE_FILE: &str = "Cannot write file:";
pub const ERR_GET_FILENAME: &str = "Cannot get filename";

// Regex Patterns
pub const HTML_COMMENT_REGEX_STR: &str = r"<!--[\s\S]*?-->";
pub const SCRIPT_TAG_REGEX_STR: &str = r"(?s)<script[^>]*>(.*?)</script>";
pub const STYLE_TAG_REGEX_STR: &str = r"(?s)<style[^>]*>(.*?)</style>";

pub fn html_comment_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(HTML_COMMENT_REGEX_STR).unwrap())
}

pub fn script_tag_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(SCRIPT_TAG_REGEX_STR).unwrap())
}

pub fn style_tag_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(STYLE_TAG_REGEX_STR).unwrap())
}
