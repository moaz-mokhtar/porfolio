/// Module to get data from local storage json file to make change and update data easy
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Portfolio {
    pub about: Vec<String>,
    pub skills: Skills,
    pub tools: Tools,
    pub projects: Vec<Project>,
    /// List of professional Experiences
    pub experiences: Vec<Experience>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Skills(Vec<String>);

// TODO 2023-08-17: Refactor to include tool's name, icon, and link
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Tools(Vec<String>);

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: Vec<String>,
    pub technologies: Option<Vec<String>>,
    pub links: Option<Vec<String>>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub description: Vec<String>,
    pub location: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// * Methods

pub fn get_portfolio() -> Portfolio {
    // TODO 2023-08-17: Add error handling when file not available or when data have error.
    serde_json::from_str::<Portfolio>(include_str!("data.json")).unwrap()
}
