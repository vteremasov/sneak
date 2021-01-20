use std::path::Path;

use crate::lib::files::get_todo_issues;
use crate::lib::ui::ui_get_to_report;
use crate::remote_target::pivotal_tracker::Pivotal;

pub mod lib;
pub mod remote_target;
pub mod util;

fn main() {
    let pivotal = Pivotal::new("".to_owned());
    let p = Path::new(".");
    let issues = get_todo_issues(p);
    let to_report = ui_get_to_report(issues);
}
