// src/context/list_context.rs

use std::collections::{BTreeMap};

use serde::Serialize;
use chrono::{DateTime, FixedOffset};

use super::{Pagination, TaxonomyItem};


#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ListKind {
    Home,
    Taxonomy {
        name: String,   // e.g., "tags" or "categories"
        slug: String,   // e.g., "rust", "programming"
    },
    Archive {
        year: u32,
        month: Option<u32>,
        day: Option<u32>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct ListContext {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    
    pub list_kind: ListKind,

    pub posts: Vec<PostListItem>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostListItem {
    pub title: String,
    pub url: String,
    pub date: DateTime<FixedOffset>,
    pub taxonomies: Option<BTreeMap<String, Vec<TaxonomyItem>>>,
    pub summary: Option<String>,
    pub thumbnail: Option<String>,
}
