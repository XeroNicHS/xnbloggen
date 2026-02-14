// src/content.rs

pub mod content_source;
pub mod content_loader;

pub use content_source::{ContentKind,SourceId, SourceMeta, ContentFrontMatter, ContentSource};
pub use content_source::MarkdownBody;