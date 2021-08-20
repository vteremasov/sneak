use crate::lib::types::Issue;

const SPACE_SHIFT: usize = 4;

pub fn print_todo(issue: Issue) {
    println!(
        "    {} => {}:{}:{}",
        issue.file_name,
        issue.file_path.to_str().unwrap_or_default(),
        issue.line.number,
        count_starting_whitespaces(&issue.line.text)
    );
    println!("{s:>width$}|", width = SPACE_SHIFT, s = "");
    println!(
        "{s:>width$} |{text}",
        s = issue.line.number + 100,
        width = SPACE_SHIFT - 1,
        text = issue.line.text
    );
    println!("{s:>width$}|", width = SPACE_SHIFT, s = "");
}

pub fn count_starting_whitespaces(string: &str) -> u32 {
    let mut result = 0;
    for char in string.chars() {
        if char::is_whitespace(char) {
            result += 1;
        } else {
            return result;
        }
    }

    result
}
