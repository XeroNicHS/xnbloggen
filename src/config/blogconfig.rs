// src/config/blogconfig.rs

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_yaml;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogConfigError {
    #[error("Failed to load blog configuration\n  Reason: {0}")]
    LoadBlogConfigError(String),

    #[error("Failed to parse blog configuration\n  Reason: {0}")]
    ParseBlogConfigError(String),
}

// Main Blog Configuration Structure
#[derive(Debug, Deserialize)]
pub struct BlogConfig {
    pub site: SiteConfig,
    pub author: AuthorConfig,
    pub theme: ThemeConfig,
    pub permalinks: PermalinkConfig,
    pub build: BuildConfig,
}

// Site Configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct SiteConfig {
    pub name: String,
    pub base_url: String,
    #[serde(default = "default_site_path")]
    pub path: String,
    #[serde(default = "default_site_description")]
    pub description: String,
    #[serde(default = "default_site_language")]
    pub language: String,
}

fn default_site_path() -> String { "".to_string()}
fn default_site_description() -> String { "".to_string() }
fn default_site_language() -> String { "en".to_string() }

// Author Configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorConfig {
    #[serde(default = "default_author_name")]
    pub name: String,
    #[serde(default = "default_author_email")]
    pub email: String,
}

fn default_author_name() -> String { "Author Name".to_string() }
fn default_author_email() -> String { "".to_string() }

// Theme Configuration
#[derive(Debug, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_name")]
    pub name: String,
}

fn default_theme_name() -> String { "default".to_string() }

// Permalink Configuration
#[derive(Debug, Deserialize)]
pub struct PermalinkConfig {
    #[serde(default = "default_post_permalink")]
    pub post: String,
    #[serde(default = "default_page_permalink")]
    pub page: String,
}

fn default_post_permalink() -> String { "/posts/:slug/".to_string() }
fn default_page_permalink() -> String { "/pages/:slug/".to_string() }

// Build Configuration
#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    #[serde(default = "default_clean_build")]
    pub clean: bool,
    #[serde(default = "default_include_drafts")]
    pub include_drafts: bool,
    #[serde(default = "default_rss_enabled")]
    pub rss: bool,
    #[serde(default = "default_rss_max_items")]
    pub rss_max_items: usize,
    #[serde(default = "default_sitemap_enabled")]
    pub sitemap: bool,
    #[serde(default = "default_robots_txt_enabled")]
    pub robots_txt: bool,
}

fn default_output_dir() -> String { "public".to_string() }
fn default_clean_build() -> bool { true }
fn default_include_drafts() -> bool { false }
fn default_rss_enabled() -> bool { true }
fn default_rss_max_items() -> usize { 20 }
fn default_sitemap_enabled() -> bool { true }
fn default_robots_txt_enabled() -> bool { true }

impl BlogConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, BlogConfigError> {
        let config_content = fs::read_to_string(path)
            .map_err(|e| BlogConfigError::LoadBlogConfigError(e.to_string()))?;

        let config = serde_yaml::from_str(&config_content)
            .map_err(|e| BlogConfigError::ParseBlogConfigError(e.to_string()))?;        

        Ok(config)
    }
}