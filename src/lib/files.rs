use regex::Regex;
use std::ffi::OsString;
use std::fs;
use std::fs::{DirEntry, FileType};
use std::path::{Path, PathBuf};

use crate::lib::types::Issue;

pub fn is_excluded(path: &Path) -> bool {
    // TODO: Read excluded paths from .gitignores
    // TODO: Read excluded paths from command params
    path.ends_with(".gitignore")
        || path.ends_with("target")
        || path.ends_with(".git")
        || path.ends_with(".idea")
}

pub fn list_files(path: &Path) -> Vec<DirEntry> {
    let mut result = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path().clone();
        if is_excluded(&path) {
            continue;
        }
        if path.is_dir() {
            result.extend(list_files(&path));
        } else {
            result.push(entry);
        }
    }

    result
}

pub fn search_in_file(path: &Path) -> Vec<String> {
    let pattern: Regex = Regex::new("(.*)TODO:(.*)").unwrap();
    let content = fs::read_to_string(path).unwrap();

    content
        .lines()
        .filter(|line| pattern.is_match(line))
        .map(|line| String::from(line))
        .collect()
}

pub fn get_todo_issues(path: &Path) -> Vec<Issue> {
    let files = list_files(path);
    let mut result = vec![];
    for file in files {
        let todos = search_in_file(file.path().as_path());

        for todo in todos {
            result.push(Issue {
                file_path: file.path(),
                file_name: String::from(file.file_name().to_str().unwrap()),
                file_type: file.file_type().unwrap(),
                todo,
            })
        }
    }

    result
}
