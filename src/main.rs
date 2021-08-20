use std::path::Path;

use crate::lib::files::get_todo_issues;
use crate::lib::print::print_todo;
use crate::remote_target::pivotal_tracker::Pivotal;

pub mod lib;
pub mod remote_target;

// TODO: get rid of unwraps and write a safe code
fn main() {
    let pivotal = Pivotal::new("ec566b8f292b1149a485e1cd7898a8ea".to_owned());
    let p = Path::new(".");
    let issues = get_todo_issues(p);
    let projects = pivotal.get_projects();
    println!("{:?}", projects);
    for issue in issues {
        println!("{:?}", issue);
        print_todo(issue);
    }
}
