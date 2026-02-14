// src/config/themeconfig.rs

use std::fs;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_yaml;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("Theme manifest file not found\n  Path: {path}\n  Expected: theme.yaml")]
    ThemeManifestNotFound {
        path: PathBuf
    },

    #[error("Failed to load theme manifest file\n  Path: {path}\n  Reason: {source}")]
    LoadThemeManifestError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse theme manifest YAML\n  Path: {path}\n  Reason: {source}")]
    ParseThemeManifestError {
        path: PathBuf,
        source: serde_yaml::Error,
    },
}

#[derive(Debug)]
pub struct ThemePackage {
    pub name: String,
    pub templates_dir: PathBuf,     // themes/<theme_name>/templates/
    pub assets_dir: PathBuf,        // themes/<theme_name>/assets/

    pub manifest: ThemeManifest,
}

#[derive(Debug, Deserialize)]
pub struct ThemeManifest {
    pub meta: ThemeMeta,
    pub template_default: ThemeTemplateDefault,

    #[serde(default)]
    pub template_extra: Vec<ThemeTemplateExtra>,

    #[serde(default)]
    pub pagination: ThemePagination,

    #[serde(default)]
    pub taxonomies: Vec<TaxonomyConfig>,

    #[serde(default = "default_archives")]
    pub archives: Vec<ArchiveConfig>,

    #[serde(default)]
    pub recent_posts: RecentPostsConfig,

    #[serde(flatten)]
    pub others: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeMeta {
    pub name: String,
    #[serde(default = "default_theme_version")]
    pub version: String,
    #[serde(default = "default_theme_author")]
    pub author: String,
    #[serde(default = "default_theme_description")]
    pub description: String,
}

fn default_theme_version() -> String { "0.1.0".to_string() }
fn default_theme_author() -> String { "xnBlogGen".to_string() }
fn default_theme_description() -> String { "A theme for xnBlogGen".to_string() }

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeTemplateDefault {
    #[serde(default = "default_theme_home")]
    pub home: String,
    #[serde(default = "default_theme_list")]
    pub list: String,
    #[serde(default = "default_theme_post")]
    pub post: String,
    #[serde(default = "default_theme_page")]
    pub page: String,
}

fn default_theme_home() -> String { "home.html".to_string() }
fn default_theme_list() -> String { "list.html".to_string() }
fn default_theme_post() -> String { "post.html".to_string() }
fn default_theme_page() -> String { "page.html".to_string() }

fn default_output_filename() -> String { "index.html".to_string() }

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeTemplateExtra {
    #[serde(default)]
    pub name: Option<String>,
    pub file: String,
    pub url: String,
    #[serde(default = "default_output_filename")]
    pub output: String,
}

#[derive(Debug, Deserialize)]
pub struct ThemePagination {
    #[serde(default = "default_per_page")]
    pub default: usize,
}

impl Default for ThemePagination {
    fn default() -> Self {
        ThemePagination {
            default: default_per_page(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaxonomyConfig {
    pub name: String,           // e.g., "tags", "categories" - Front Matter key
    pub label: String,          // e.g., "Tag", "Category" - Display label
    pub permalink: String,      // e.g., "/tags/:slug/", "/categories/:slug/"

    #[serde(default = "default_per_page")]
    pub per_page: usize,

    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ArchiveKind {
    Yearly,
    Monthly,
    Daily,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArchiveConfig {
    pub kind: ArchiveKind,      // e.g., "Yearly", "Monthly", "Daily"
    pub permalink: String,      // e.g., "/archives/:year/:month/"

    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

fn default_archives() -> Vec<ArchiveConfig> {
    vec![
        ArchiveConfig {
            kind: ArchiveKind::Monthly,
            permalink: "/archives/:year/:month/".to_string(),
            per_page: default_per_page(),
        },
    ]
}

fn default_per_page() -> usize { 10 }
fn default_enabled() -> bool { true }

#[derive(Debug, Deserialize)]
pub struct RecentPostsConfig {
    #[serde(default = "default_recent_posts_count")]
    pub count: usize,
}

impl Default for RecentPostsConfig {
    fn default() -> Self {
        RecentPostsConfig {
            count: default_recent_posts_count(),
        }
    }
}

fn default_recent_posts_count() -> usize { 10 }


impl ThemePackage {
    pub fn load_from_dir(theme_dir: &Path) -> Result<Self, ThemeError> {
        let manifest_path = theme_dir.join("theme.yaml");
        if !manifest_path.is_file() {
            return Err(ThemeError::ThemeManifestNotFound { path: theme_dir.to_path_buf() });
        }

        let yaml = fs::read_to_string(&manifest_path)
            .map_err(|e| ThemeError::LoadThemeManifestError {
                path: manifest_path.clone(),
                source: e,
            })?;

        let manifest: ThemeManifest = serde_yaml::from_str(&yaml)
            .map_err(|e| ThemeError::ParseThemeManifestError {
                path: manifest_path.clone(),
                source: e,
            })?;

        let templates_dir = theme_dir.join("templates");
        let assets_dir = theme_dir.join("assets");

        Ok(Self {
            name: manifest.meta.name.clone(),
            templates_dir,
            assets_dir,
            manifest,
        })
    }
}