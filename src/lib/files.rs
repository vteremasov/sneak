use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use crate::lib::types::{CodeLine, Issue};

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
        } else if path.is_file() {
            result.push(entry);
        } else {
            continue;
        }
    }

    result
}

pub fn search_in_file(path: &Path) -> Vec<CodeLine> {
    let pattern: Regex = Regex::new("[//|#|;;]\\s*TODO:(?P<TEXT>.+)").unwrap();
    let content = fs::read_to_string(path).unwrap();

    content
        .lines()
        .enumerate()
        .map(|(idx, line)| CodeLine {
            number: idx as u64 + 1,
            text: String::from(line),
        })
        .filter(|line| pattern.is_match(&line.text))
        .collect()
}

pub fn is_reported(todo: &str) -> bool {
    let pattern: Regex = Regex::new("[//|#|;;]\\s*TODO(?P<REPORT>\\(reported:\\s*(http(s)?://.*\\.[\\w]+(\\?.*)?)\\)):(?P<TEXT>.+)").unwrap();
    pattern.is_match(todo)
}

pub fn get_todo_issues(path: &Path) -> Vec<Issue> {
    let files = list_files(path);

    files
        .par_iter()
        .map(|file| {
            search_in_file(file.path().as_path())
                .iter()
                .map(|todo| Issue {
                    file_path: file.path(),
                    file_name: String::from(file.file_name().to_str().unwrap_or_default()),
                    line: todo.clone(),
                    reported: is_reported(&todo.text),
                })
                .collect::<Vec<Issue>>()
        })
        .flatten()
        .collect()
}
