// src/content/loader.rs

use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;
use serde_yaml;

use super::{ContentKind, SourceId, SourceMeta, ContentFrontMatter, ContentSource};
use super::MarkdownBody;

#[derive(Error, Debug)]
pub enum ContentLoaderError {
    #[error("IO error\n  Path: {path}\n  Reason: {source}")]
    Io { path: PathBuf, source: std::io::Error },

    #[error("Invalid front matter fence\n  Path: {path}\n  Expected: YAML format with '---' delimiters")]
    InvalidFrontMatterFence { path: PathBuf },

    #[error("YAML front matter parse error\n  Path: {path}\n  Reason: {source}")]
    YamlParseError { path: PathBuf, source: serde_yaml::Error },

    #[error("Missing required front matter field\n  Path: {path}\n  Field: {field}")]
    MissingField { path: PathBuf, field: &'static str },
}

pub fn load_all_contents(content_root: &Path) -> Result<Vec<ContentSource>, ContentLoaderError> {
    let mut out = Vec::new();

    let content_data = [
        (content_root.join("posts"), ContentKind::Post),
        (content_root.join("pages"), ContentKind::Page),
    ];

    for (content_dir, content_kind) in &content_data {
        let (md_files, image_files) = list_content_files(content_dir)?;
        for md_file in md_files {
            out.push(load_content_file(&md_file, content_root, *content_kind, &image_files)?);
        }
    }

    Ok(out)
}

fn list_content_files(dir: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), ContentLoaderError> {
    let mut md_files = Vec::new();
    let mut image_files = Vec::new();

    if !dir.is_dir() {
        return Ok((md_files, image_files));
    }

    for entry in fs::read_dir(&dir)
        .map_err(|e| ContentLoaderError::Io { path: dir.to_path_buf(), source: e })? {

        let entry = entry.map_err(|e| ContentLoaderError::Io { path: dir.to_path_buf(), source: e })?;
        let path = entry.path();

        if path.is_dir() {
            let index_md = path.join("index.md");
            if index_md.is_file() {
                md_files.push(index_md);

                for sub_entry in fs::read_dir(&path)
                    .map_err(|e| ContentLoaderError::Io { path: path.to_path_buf(), source: e })? {

                    let sub_entry = sub_entry.map_err(|e| ContentLoaderError::Io { path: path.to_path_buf(), source: e })?;
                    let sub_path = sub_entry.path();
                    if !sub_path.is_file() { continue; }

                    match sub_path.extension().and_then(|s| s.to_str()) {
                        Some(ext) if is_image_ext(ext) => image_files.push(sub_path),
                        _ => {},
                    }
                }
            }
            continue;
        }

        if !path.is_file() { continue; }

        match path.extension().and_then(|s| s.to_str()) {
            Some("md") => md_files.push(path),
            Some(ext) if is_image_ext(ext) => image_files.push(path),
            _ => {},
        }
    }

    md_files.sort();
    image_files.sort();

    Ok((md_files, image_files))
}

fn read_file(path: &Path) -> Result<String, ContentLoaderError> {
    fs::read_to_string(path).map_err(|e| ContentLoaderError::Io { path: path.to_path_buf(), source: e })
}

//------------------------------------------------------------------------------
// ---
// yaml...
// ---
// markdown...
//------------------------------------------------------------------------------
fn split_front_matter(input: &str, path: &Path) -> Result<(String, String), ContentLoaderError> {
    let mut lines = input.lines();

    let first = lines.next().unwrap_or("");
    if first.trim() != "---" {
        return Err(ContentLoaderError::InvalidFrontMatterFence { path: path.to_path_buf() });
    }

    let mut yaml = String::new();
    let mut found_end = false;

    for line in lines.by_ref() {
        if line.trim() == "---" {
            found_end = true;
            break;
        }
        yaml.push_str(line);
        yaml.push('\n');
    }

    if !found_end {
        return Err(ContentLoaderError::InvalidFrontMatterFence { path: path.to_path_buf() });
    }

    let markdown = lines.collect::<Vec<_>>().join("\n");
    Ok((yaml, markdown))
}

fn file_mtime_unix(path: &Path) -> Option<i64> {
    let md = fs::metadata(path).ok()?;
    let mtime = md.modified().ok()?;
    Some(mtime.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64)
}

fn make_source_id(content_root: &Path, file_path: &Path) -> SourceId {
    let rel = file_path.strip_prefix(content_root).unwrap_or(file_path);
    let mut s = rel.to_string_lossy().replace('\\', "/");
    if s.ends_with(".md") {
        s.truncate(s.len() - 3);
    }
    SourceId(s)
}

fn load_content_file(path: &Path, content_root: &Path, kind: ContentKind, all_images: &[PathBuf]) -> Result<ContentSource, ContentLoaderError> {
    let raw = read_file(path)?;
    let (yaml, markdown) = split_front_matter(&raw, path)?;

    let fm: ContentFrontMatter =
        serde_yaml::from_str(&yaml).map_err(|e| ContentLoaderError::YamlParseError { path: path.to_path_buf(), source: e })?;

    if fm.title.trim().is_empty() {
        return Err(ContentLoaderError::MissingField { path: path.to_path_buf(), field: "title" });
    }

    let meta = SourceMeta {
        id: make_source_id(content_root, path),
        source_path: path.to_path_buf(),
        source_mtime_unix: file_mtime_unix(path),
    };

    let images = filter_content_images(path, &fm, all_images);

    Ok(ContentSource { kind, meta, front_matter: fm, body: MarkdownBody { markdown }, images } )
}

fn filter_content_images(md_path: &Path, front_matter: &ContentFrontMatter, all_images: &[PathBuf]) -> Vec<PathBuf> {
    // Pattern 1: Folder Images (e.g., content/posts/my-post/index.md -> content/posts/my-post/*)
    if md_path.file_name().and_then(|s| s.to_str()) == Some("index.md") {
        if let Some(parent_dir) = md_path.parent() {
            return all_images.iter()
                .filter(|img_path| img_path.parent() == Some(parent_dir))
                .cloned()
                .collect();
        }
    }

    // Pattern 2: Flattened Images with Slug(optional Date) (e.g., content/posts/2023-10-01-my-post.md -> content/posts/my-post-* or content/posts/2023-10-01-my-post-*)
    let date = front_matter.date.format("%Y-%m-%d").to_string();
    let title_slug = if let Some(slug) = &front_matter.slug {
        slug.clone()
    } else {
        md_path.file_stem()
            .and_then(|s| s.to_str().map(|s| s.to_lowercase()))
            .map(|s| {
                let s = s.trim_start_matches(&date).trim_start_matches('-');
                s.to_string()
            })
            .unwrap_or_default()
    };

    let slug_prefix = format!("{}-", title_slug);
    let date_slug_prefix = format!("{}-{}-", date, title_slug);

    all_images.iter()
        .filter(|img_path| {
            img_path.file_stem()
                .and_then(|s| s.to_str().map(|s| s.to_lowercase()))
                .map_or(false, |stem| {
                    stem.starts_with(&slug_prefix) || stem.starts_with(&date_slug_prefix)
                })
        })
        .cloned()
        .collect()
}

fn is_image_ext(ext: &str) -> bool {
    matches!(ext, "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "avif" | "bmp" | "ico" | "tiff" | "tif")
}