use std::ffi::OsString;
use std::fs::FileType;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Issue {
    pub todo: String,
    pub file_name: String,
    pub file_path: PathBuf,
    pub file_type: FileType,
}
