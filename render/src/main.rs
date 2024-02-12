use std::fs;

use askama::Template;
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

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

/// A tag is a special kind of string that
/// - is lowercase
/// - has no whitespace
/// - has no special characters except for `-`
/// - has no leading or trailing `-`
/// - has no consecutive `-`
/// - has no more than 50 characters
/// - is not empty
/// - only contains ASCII characters
/// - does not contain numbers
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Tag(String);

impl TryFrom<String> for Tag {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Tag cannot be empty");
        }

        if value.len() > 50 {
            return Err("Tag cannot be longer than 50 characters");
        }

        if value.contains(|c: char| !c.is_ascii_lowercase() && c != '-') {
            return Err("Tag can only contain lowercase ASCII characters");
        }

        if value.contains(|c: char| c.is_ascii_digit()) {
            return Err("Tag cannot contain numbers");
        }

        if value.contains(|c: char| !c.is_ascii() && c != '-') {
            return Err("Tag can only contain ASCII characters and hyphens");
        }

        if value.starts_with('-') || value.ends_with('-') {
            return Err("Tag cannot start or end with a hyphen");
        }

        if value.contains("--") {
            return Err("Tag cannot contain consecutive hyphens");
        }

        if value.contains(char::is_whitespace) {
            return Err("Tag cannot contain whitespace");
        }

        Ok(Tag(value))
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Tag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Tag::try_from(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
enum Difficulty {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "beginner")]
    Beginner,
    #[serde(rename = "intermediate")]
    Intermediate,
    #[serde(rename = "advanced")]
    Advanced,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
enum InteractivityLevel {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

#[derive(Debug, Deserialize, Clone, Serialize, Eq, PartialEq, Ord, PartialOrd)]
enum Category {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "workshop")]
    Workshop,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "talk")]
    Talk,
    #[serde(rename = "forum")]
    Forum,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Resource {
    title: String,
    url: Url,
    description: String,
    tags: Vec<Tag>,
    official: bool,
    year: usize,
    #[serde(rename = "difficultyLevel")]
    difficulty_level: Difficulty,
    duration: Option<String>,
    #[serde(rename = "interactivityLevel")]
    interactivity_level: InteractivityLevel,
    free: bool,
    category: Category,
}

type Resources = Vec<Resource>;

type YearMap = IndexMap<usize, Resources>;

fn group_by_year(resources: &Resources, category: Category) -> YearMap {
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

fn sort_by_title(resources: &Resources, category: Category) -> Resources {
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
        projects: sort_by_title(&resources, Category::Project),
        workshops: sort_by_title(&resources, Category::Workshop),
        books: sort_by_title(&resources, Category::Book),
        articles: group_by_year(&resources, Category::Article),
        talks: group_by_year(&resources, Category::Talk),
        forum: group_by_year(&resources, Category::Forum),
    };

    fs::write("README.md", readme.render()?)?;

    Ok(())
}
