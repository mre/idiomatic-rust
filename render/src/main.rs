use std::fs;

use askama::Template;
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "README.md")]
struct ReadmeTemplate {
    projects: Vec<Resource>,
    workshops: Vec<Resource>,
    books: Vec<Resource>,
    articles: YearMap,
    talks: YearMap,
    forum: YearMap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resource {
    title: String,
    url: String,
    description: String,
    tags: Vec<String>,
    official: bool,
    year: usize,
    #[serde(rename = "difficultyLevel")]
    difficulty_level: String,
    duration: Option<String>,
    #[serde(rename = "interactivityLevel")]
    interactivity_level: String,
    free: bool,
    category: String,
}

type Resources = Vec<Resource>;

type YearMap = IndexMap<usize, Resources>;

fn group_by_year(resources: &Resources, category: &str) -> YearMap {
    resources
        .iter()
        .filter(|r| r.category == category)
        .sorted_by_key(|r| r.year)
        .rev()
        .fold(YearMap::new(), |mut map, r| {
            map.entry(r.year).or_insert_with(Vec::new).push(r.clone());
            map
        })
}

fn sort_by_title(resources: &Resources, category: &str) -> Resources {
    resources
        .iter()
        .filter(|r| r.category == category)
        .sorted_by_key(|r| r.title.to_lowercase())
        .cloned()
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("resources.json")?;
    let resources: Resources = serde_json::from_reader(file)?;

    let readme = ReadmeTemplate {
        projects: sort_by_title(&resources, "project"),
        workshops: sort_by_title(&resources, "workshop"),
        books: sort_by_title(&resources, "book"),
        articles: group_by_year(&resources, "article"),
        talks: group_by_year(&resources, "talk"),
        forum: group_by_year(&resources, "forum"),
    };

    fs::write("README.md", readme.render()?)?;

    Ok(())
}
