// src/context/common_context.rs

use std::collections::{BTreeMap};

use serde::Serialize;
use serde_yaml;

use crate::{config::theme::ArchiveKind, context::PostListItem};

#[derive(Debug, Clone, Serialize)]
pub struct TaxonomyItem {
    pub label: String,
    pub url: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArchiveItem {
    pub label: String,
    pub kind: ArchiveKind,
    pub year: u32,
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub url: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageLink {
    pub number: usize,
    pub url: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct NavLink {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total_items: usize,
    pub total_pages: usize,
    pub has_prev: bool,
    pub has_next: bool,
    pub prev: Option<NavLink>,
    pub next: Option<NavLink>,
    pub first: Option<NavLink>,
    pub last: Option<NavLink>,
    pub pages: Vec<PageLink>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SiteContext {
    pub title: String,
    pub base_url: String,
    pub path: String,
    pub description: String,
    pub language: String,

    pub author: Option<String>,
    pub email: Option<String>,

    pub taxonomies: BTreeMap<String, Vec<TaxonomyItem>>,
    pub archives: Vec<ArchiveItem>,
    pub recent_posts: Vec<PostListItem>,

    pub theme: BTreeMap<String, serde_yaml::Value>,
}