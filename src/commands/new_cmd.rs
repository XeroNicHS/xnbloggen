// src/commands/new.rs

use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;
use chrono::prelude::*;

use crate::context::context_builder;
use crate::utils::output;

#[derive(Debug)]
pub enum NewKind {
    Post,
    Page,
}

#[derive(Error, Debug)]
pub enum NewError {
    #[error("IO error\n  Path: {path}\n  Reason: {source}")]
    Io{ path: PathBuf, source: std::io::Error },

    #[error("Blog project not found\n  Path: {path}\n  Expected: blogconfig.yaml")]
    ProjectNotFound{ path: PathBuf },

    #[error("Content directory not found\n  Path: {path}\n  Expected: content/posts/")]
    PostDirNotFound{ path: PathBuf },

    #[error("Post already exists\n  Path: {path}")]
    PostAlreadyExists{ path: PathBuf },

    #[error("Content directory not found\n  Path: {path}\n  Expected: content/pages/")]
    PageDirNotFound{ path: PathBuf },

    #[error("Page already exists\n  Path: {path}")]
    PageAlreadyExists{ path: PathBuf },
}

pub fn run(title: &str, kind: NewKind, root: &str) -> Result<(), NewError> {
    let project_path = PathBuf::from(root);
    let config_path = project_path.join("blogconfig.yaml");

    if !config_path.is_file() {
        return Err(NewError::ProjectNotFound{ path: project_path.clone() });
    }

    match kind {
        NewKind::Post => create_new_post(title, &project_path.as_path()),
        NewKind::Page => create_new_page(title, &project_path.as_path()),
    }
}

fn create_new_post(title: &str, project_path: &Path) -> Result<(), NewError> {
    let content_post_dir = project_path.join("content").join("posts");
    if !content_post_dir.exists() {
        return Err(NewError::PostDirNotFound{ path: content_post_dir.clone() });
    }

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let new_post_filename = format!("{}-{}.md", current_date, title.replace(" ", "-"));
    let new_post_path = content_post_dir.join(&new_post_filename);
    if new_post_path.exists() {
        return Err(NewError::PostAlreadyExists{ path: new_post_path.clone() });
    }

    let new_post_content = format!(r#"---
title: "{title}"
slug: "{slug}"
date: "{datetime_str}"

taxonomies:
  categories: ["Uncategorized"]
  tags: ["tag1", "tag2"]

summary: "Write a brief summary of your post here."
thumbnail: ""

draft: false
---

Write your post content here.
"#,
        title = title,
        slug = context_builder::slugify(title),
        datetime_str = Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
    );

    fs::write(&new_post_path, new_post_content)
        .map_err(|e| NewError::Io { path: new_post_path.clone(), source: e })?;

    output::success("New post created");
    output::print_path(&new_post_path.display().to_string());

    Ok(())
}

fn create_new_page(title: &str, project_path: &Path) -> Result<(), NewError> {
    let content_page_dir = project_path.join("content").join("pages");
    if !content_page_dir.exists() {
        return Err(NewError::PageDirNotFound{ path: content_page_dir.clone() });
    }

    let new_page_filename = format!("{}.md", title.replace(" ", "-"));
    let new_page_path = content_page_dir.join(&new_page_filename);
    if new_page_path.exists() {
        return Err(NewError::PageAlreadyExists{ path: new_page_path.clone() });
    }

    let new_page_content = format!(r#"---
title: "{title}"
slug: "{slug}"
date: "{datetime_str}"

draft: false
---

Write your page content here.
"#,
        title = title,
        slug = context_builder::slugify(title),
        datetime_str = Local::now().format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
    );

    fs::write(&new_page_path, new_page_content)
        .map_err(|e| NewError::Io { path: new_page_path.clone(), source: e })?;

    output::success("New page created");
    output::print_path(&new_page_path.display().to_string());

    Ok(())
}