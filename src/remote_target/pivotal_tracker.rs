use reqwest;
use serde_json::json;
use std::collections::HashMap;

const API_URL: &str = "https://www.pivotaltracker.com/services/v5";
const AUTH_HEADER: &str = "X-TrackerToken";

pub struct Pivotal {
    token: String,
}

impl Pivotal {
    pub fn new(token: String) -> Self {
        Pivotal { token }
    }

    pub fn get_projects(&self) -> HashMap<String, String> {
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
        project_id: String,
        title: String,
        description: String,
    ) -> HashMap<String, String> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/project/{}/story", API_URL, project_id);
        let body = json!({
            "name": title,
            "description": description,
            "story_type": "feature",
            "labels": vec!["todo"]
        });

        client
            .post(&url)
            .header(AUTH_HEADER, "ec566b8f292b1149a485e1cd7898a8ea")
            .body(body.to_string())
            .send()
            .unwrap()
            .json()
            .unwrap()
    }
}
