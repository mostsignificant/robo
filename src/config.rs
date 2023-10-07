use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    version: Option<String>,
    on: On,
    jobs: HashMap<String, Job>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
enum On {
    Schedule(Vec<Cron>),
    Event,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
struct Cron {
    cron: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Job {
    steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
enum Step {
    Collect(Collect),
    Store(Store),
    Run(String),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(transparent)]
struct Collect {
    collect: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(transparent)]
struct Store {
    store: String,
}
