use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Issue {
    pub line: CodeLine,
    pub file_name: String,
    pub file_path: PathBuf,
    pub reported: bool,
}

#[derive(Debug, Clone)]
pub struct CodeLine {
    pub number: u64,
    pub text: String,
}
