use crate::lib::types::Issue;
use reqwest;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

const API_URL: &str = "https://www.pivotaltracker.com/services/v5";
const AUTH_HEADER: &str = "X-TrackerToken";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    id: i64,
    kind: String,
    name: String,
    iteration_length: i64,
    week_start_day: String,
    velocity_averaged_over: i64,
    public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    id: i64,
    kind: String,
    story_type: String,
    name: String,
    description: String,
    current_state: String,
}

pub struct Pivotal {
    token: String,
}

impl Pivotal {
    pub fn new(token: String) -> Self {
        Pivotal { token }
    }

    pub fn get_projects(&self) -> Vec<Project> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/projects", API_URL);

        client
            .get(&url)
            .header(AUTH_HEADER, &self.token)
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    pub fn create_story(
        &self,
        project_id: &str,
        title: String,
        description: String,
    ) -> Result<Story, Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/projects/{}/stories", API_URL, project_id);
        let body = json!({
            "name": title,
            "description": description,
            "story_type": "feature",
            "labels": vec!["todo"]
        });

        client
            .post(&url)
            .header(AUTH_HEADER, "ec566b8f292b1149a485e1cd7898a8ea")
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .unwrap()
            .json()
    }

    pub fn report(&self, project_id: &str, issue: &Issue) -> Result<(), String> {
        let title = issue.line.text.trim().replace("TODO: ", "");

        match self.create_story(project_id, title, "TBD".to_owned()) {
            Ok(res) => {
                println!("{:?}", res);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
