use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use crate::lib::types::{CodeLine, Issue};

pub fn is(path: &Path, patterns: Vec<Regex>) -> bool {
    // TODO: Read excluded paths from .gitignores
    let path_str = path.to_str().unwrap();

    for p in patterns.clone() {
        if p.is_match(path_str) {
            return true;
        }
    }

    false
}

pub fn list_files(path: &Path, excluded: Vec<Regex>, included: Vec<Regex>) -> Vec<DirEntry> {
    // TODO: get rid of unwraps here
    let mut result = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path().clone();
        if is(&path, excluded.clone()) && !is(&path, included.clone()) {
            continue;
        }
        if path.is_dir() {
            result.extend(list_files(&path, excluded.clone(), included.clone()));
        } else if path.is_file() {
            result.push(entry);
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
    let mut excluded = vec![
        Regex::new(".*\\.git$").unwrap(),
        Regex::new(".*\\.idea$").unwrap(),
    ];
    excluded.extend(list_excludes(path, git_exclude));
    let included = list_excludes(path, git_include);
    let files = list_files(path, excluded, included);

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

fn git_exclude(line: &&str) -> bool {
    let string = String::from(*line);
    !string.starts_with('#') && !string.starts_with('!')
}

fn git_include(line: &&str) -> bool {
    let string = String::from(*line);
    string.starts_with('!')
}

fn list_excludes(path: &Path, filter_out: fn(line: &&str) -> bool) -> Vec<Regex> {
    let excl = vec![Regex::new(".*").unwrap()];
    let incl = vec![Regex::new(".*\\.gitignore").unwrap()];
    let git_ignores = list_files(path, excl, incl);

    let mut result = vec![];

    for file in git_ignores {
        result.extend(
            fs::read_to_string(file.path().as_path())
                .unwrap()
                .lines()
                .filter(filter_out)
                .map(gitignore_pattern_to_reg)
                .map(|s| Regex::new(&s).unwrap())
                .collect::<Vec<Regex>>(),
        )
    }

    result
}

fn gitignore_pattern_to_reg(pattern: &str) -> String {
    String::from(pattern)
        .split("")
        .map(|el| match el {
            "*" => String::from(".*"),
            "?" => String::from("."),
            "." | "\\" | "(" | ")" => String::from("\\") + el,
            _ => String::from(el),
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;
    use regex;

    #[test]
    fn test_gitignore_pattern_to_reg() {
        assert_eq!(gitignore_pattern_to_reg("*"), String::from(".*"));
        assert_eq!(
            gitignore_pattern_to_reg("asdf?asdf"),
            String::from("asdf.asdf")
        );
        assert_eq!(
            gitignore_pattern_to_reg("**/*.js"),
            String::from(".*.*/.*\\.js")
        );

        assert!(regex::Regex::new(&gitignore_pattern_to_reg("**/*.js"))
            .unwrap()
            .is_match("test/test/test.js"));
    }
}
