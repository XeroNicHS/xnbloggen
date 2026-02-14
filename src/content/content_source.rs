// src/content/source.rs

use std::path::PathBuf;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_yaml;
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentKind {
    Post,
    Page,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceMeta {
    pub id: SourceId,
    pub source_path: PathBuf,

    // Option
    #[serde(default)]
    pub source_mtime_unix: Option<i64>,
}

// Content Front Matter Structures
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentFrontMatter {
    pub title: String,

    #[serde(default)]
    pub slug: Option<String>,

    #[serde(default)]
    pub date: DateTime<FixedOffset>,

    #[serde(default)]
    pub updated: Option<DateTime<FixedOffset>>,    

    #[serde(default)]
    pub draft: bool,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub language: Option<String>,

    #[serde(default)]
    pub extra: BTreeMap<String, serde_yaml::Value>,

    // Only for Post Type
    #[serde(default)]
    pub taxonomies: Option<BTreeMap<String, Vec<String>>>,

    #[serde(default)]
    pub summary: Option<String>,

    #[serde(default)]
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarkdownBody {
    pub markdown: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentSource {
    pub kind: ContentKind,
    pub meta: SourceMeta,
    pub front_matter: ContentFrontMatter,
    pub body: MarkdownBody,

    #[serde(default)]
    pub images: Vec<PathBuf>,
}
