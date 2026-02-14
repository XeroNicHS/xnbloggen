// src/context/content_context.rs

use std::collections::{BTreeMap};

use serde::Serialize;
use serde_yaml::Value;
use chrono::{DateTime, FixedOffset};

use crate::content::content_source::{ContentKind};

use super::{TaxonomyItem, NavLink};


#[derive(Debug, Clone, Serialize)]
pub struct ContentContext {
    pub kind: ContentKind,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub language: Option<String>,

    pub date: DateTime<FixedOffset>,
    pub updated: Option<DateTime<FixedOffset>>,

    pub content_html: String,

    pub extra: BTreeMap<String, Value>,

    pub taxonomies: Option<BTreeMap<String, Vec<TaxonomyItem>>>,

    pub summary: Option<String>,
    pub thumbnail: Option<String>,    

    pub prev: Option<NavLink>,
    pub next: Option<NavLink>,
}
